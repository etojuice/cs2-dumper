use crate::error::Result;
use crate::remote::Process;

use super::SchemaSystemTypeScope;

pub struct SchemaSystem<'a> {
    process: &'a Process,
    address: usize,
}

impl<'a> SchemaSystem<'a> {
    pub fn new(process: &'a Process) -> Result<Self> {
        let mut address = process.find_pattern(
            "schemasystem.dll",
            "48 8D 0D ? ? ? ? E9 ? ? ? ? CC CC CC CC 48 8D 0D ? ? ? ? E9 ? ? ? ? CC CC CC CC 48 83 EC 28"
        )?;

        address = process.resolve_rip(address)?;

        Ok(Self { process, address })
    }

    pub fn type_scopes(&self) -> Result<Vec<SchemaSystemTypeScope>> {
        let size = self.process.read_memory::<u32>(self.address + 0x190)?;
        let data = self.process.read_memory::<usize>(self.address + 0x198)?;

        let mut addresses: Vec<usize> = vec![0; size as usize];

        self.process.read_memory_raw(
            data,
            addresses.as_mut_ptr() as *mut _,
            addresses.len() * std::mem::size_of::<usize>(),
        )?;

        let type_scopes: Vec<SchemaSystemTypeScope> = addresses
            .iter()
            .map(|&address| SchemaSystemTypeScope::new(self.process, address))
            .collect();

        Ok(type_scopes)
    }
}
