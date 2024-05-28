use crate::{Backend, BulkString, RespArray, RespFrame};

use super::{
    extract_args, extract_string, validate_commands, validate_nums_of_argument, CommandError,
    CommandExecutor,
};

#[derive(Debug, PartialEq)]
pub struct Sadd {
    key: String,
    fields: Vec<String>,
}
#[derive(Debug, PartialEq)]
pub struct SMembers {
    key: String,
    sort: bool,
}

impl CommandExecutor for Sadd {
    fn execute(self, backend: &crate::Backend) -> RespFrame {
        backend.sadd(self.key, self.fields)
    }
}
impl RespFrame {
    fn from_string(s: String) -> Self {
        RespFrame::BulkString(BulkString::new(Some(s.into_bytes())))
    }
}

impl CommandExecutor for SMembers {
    fn execute(self, backend: &Backend) -> RespFrame {
        match backend.smembers(&self.key) {
            Some(values) => {
                let mut array = Vec::with_capacity(values.len());
                for value in values.iter() {
                    array.push((
                        value.key().to_owned(),
                        BulkString::from((*value).clone()).into(),
                    ));
                }
                if self.sort {
                    array.sort_by(|a, b| a.0.cmp(&b.0));
                }
                // 不需要返回 key
                let ret = array
                    .into_iter()
                    .map(|(_, v)| v)
                    .collect::<Vec<RespFrame>>();
                RespArray::new(Some(ret)).into()
            }
            None => RespArray::new(Some([])).into(),
        }
    }
}

impl TryFrom<RespArray> for SMembers {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_commands(&value, &["smembers"], 1, false)?;

        let mut args = extract_args(value, 1)?.into_iter();
        match args.next() {
            Some(RespFrame::BulkString(key)) => Ok(SMembers {
                key: String::from_utf8(key.0.unwrap())?,
                sort: true,
            }),
            _ => Err(CommandError::InvalidArgument("Invalid key".to_string())),
        }
    }
}

impl TryFrom<Vec<RespFrame>> for SMembers {
    type Error = CommandError;
    fn try_from(value: Vec<RespFrame>) -> Result<Self, Self::Error> {
        validate_nums_of_argument(&value, "smembers", value.len(), 1)?;
        let key = extract_string(value.get(0).cloned())?;
        Ok(SMembers { key, sort: true })
    }
}
impl TryFrom<Vec<RespFrame>> for Sadd {
    type Error = CommandError;

    fn try_from(value: Vec<RespFrame>) -> Result<Self, Self::Error> {
        validate_nums_of_argument(&value, "sadd", value.len(), 2)?;
        let mut frame_iter = value.into_iter();
        let key = extract_string(frame_iter.next())?;
        let mut fields = Vec::with_capacity(frame_iter.len());
        for frame in frame_iter {
            let field = extract_string(Some(frame))?;
            fields.push(field);
        }
        Ok(Sadd::new(key, fields))
    }
}

impl Sadd {
    pub fn new(key: String, fields: Vec<String>) -> Self {
        Sadd { key, fields }
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;

    use crate::{cmd::Command, RespArray, RespDecode};

    use super::*;

    #[test]
    fn test_sadd_try_from() {
        let mut buf = BytesMut::from(b"*3\r\n$4\r\nsadd\r\n$3\r\nset\r\n$3\r\none\r\n".as_slice());
        let array = RespArray::decode(&mut buf).expect("error in decode resp array");
        let sadd = Command::try_from(array).unwrap();
        assert_eq!(
            sadd,
            Command::Sadd(Sadd::new("set".to_string(), vec!["one".to_string()]))
        )
    }
}
