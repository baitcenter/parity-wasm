use std::io;
use super::{Deserialize, Serialize, Error, GlobalType, InitExpr};

pub struct GlobalEntry {
    global_type: GlobalType,
    init_expr: InitExpr,
}

impl GlobalEntry {
    pub fn global_type(&self) -> &GlobalType { &self.global_type }
    pub fn init_expr(&self) -> &InitExpr { &self.init_expr }
}

impl Deserialize for GlobalEntry {
    type Error = Error;

    fn deserialize<R: io::Read>(reader: &mut R) -> Result<Self, Self::Error> {
        let global_type = GlobalType::deserialize(reader)?;
        let init_expr = InitExpr::deserialize(reader)?;

        Ok(GlobalEntry {
            global_type: global_type,
            init_expr: init_expr,
        })
    }    
} 

impl Serialize for GlobalEntry {
    type Error = Error;

    fn serialize<W: io::Write>(self, writer: &mut W) -> Result<(), Self::Error> {
        self.global_type.serialize(writer)?;
        self.init_expr.serialize(writer)
    }
}