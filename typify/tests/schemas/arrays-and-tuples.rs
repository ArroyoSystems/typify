#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LessSimpleTwoTuple(pub (String, String));
impl std::ops::Deref for LessSimpleTwoTuple {
    type Target = (String, String);
    fn deref(&self) -> &(String, String) {
        &self.0
    }
}
impl From<LessSimpleTwoTuple> for (String, String) {
    fn from(value: LessSimpleTwoTuple) -> Self {
        value.0
    }
}
impl From<&LessSimpleTwoTuple> for LessSimpleTwoTuple {
    fn from(value: &LessSimpleTwoTuple) -> Self {
        value.clone()
    }
}
impl From<(String, String)> for LessSimpleTwoTuple {
    fn from(value: (String, String)) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SimpleTwoArray(pub [String; 2usize]);
impl std::ops::Deref for SimpleTwoArray {
    type Target = [String; 2usize];
    fn deref(&self) -> &[String; 2usize] {
        &self.0
    }
}
impl From<SimpleTwoArray> for [String; 2usize] {
    fn from(value: SimpleTwoArray) -> Self {
        value.0
    }
}
impl From<&SimpleTwoArray> for SimpleTwoArray {
    fn from(value: &SimpleTwoArray) -> Self {
        value.clone()
    }
}
impl From<[String; 2usize]> for SimpleTwoArray {
    fn from(value: [String; 2usize]) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SimpleTwoTuple(pub (String, String));
impl std::ops::Deref for SimpleTwoTuple {
    type Target = (String, String);
    fn deref(&self) -> &(String, String) {
        &self.0
    }
}
impl From<SimpleTwoTuple> for (String, String) {
    fn from(value: SimpleTwoTuple) -> Self {
        value.0
    }
}
impl From<&SimpleTwoTuple> for SimpleTwoTuple {
    fn from(value: &SimpleTwoTuple) -> Self {
        value.clone()
    }
}
impl From<(String, String)> for SimpleTwoTuple {
    fn from(value: (String, String)) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UnsimpleTwoTuple(pub (String, String));
impl std::ops::Deref for UnsimpleTwoTuple {
    type Target = (String, String);
    fn deref(&self) -> &(String, String) {
        &self.0
    }
}
impl From<UnsimpleTwoTuple> for (String, String) {
    fn from(value: UnsimpleTwoTuple) -> Self {
        value.0
    }
}
impl From<&UnsimpleTwoTuple> for UnsimpleTwoTuple {
    fn from(value: &UnsimpleTwoTuple) -> Self {
        value.clone()
    }
}
impl From<(String, String)> for UnsimpleTwoTuple {
    fn from(value: (String, String)) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct YoloTwoArray(pub [serde_json::Value; 2usize]);
impl std::ops::Deref for YoloTwoArray {
    type Target = [serde_json::Value; 2usize];
    fn deref(&self) -> &[serde_json::Value; 2usize] {
        &self.0
    }
}
impl From<YoloTwoArray> for [serde_json::Value; 2usize] {
    fn from(value: YoloTwoArray) -> Self {
        value.0
    }
}
impl From<&YoloTwoArray> for YoloTwoArray {
    fn from(value: &YoloTwoArray) -> Self {
        value.clone()
    }
}
impl From<[serde_json::Value; 2usize]> for YoloTwoArray {
    fn from(value: [serde_json::Value; 2usize]) -> Self {
        Self(value)
    }
}
fn main() {}
