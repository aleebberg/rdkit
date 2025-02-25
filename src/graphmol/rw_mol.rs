use std::fmt::Formatter;

use cxx::{let_cxx_string, SharedPtr};
use rdkit_sys::*;

use crate::ROMol;

pub struct RWMol {
    pub(crate) ptr: SharedPtr<rdkit_sys::rw_mol_ffi::RWMol>,
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum RWMolError {
    #[error("Could not convert smarts to RWMol (nullptr)")]
    UnknownConversionError,
    #[error("could not convert smarts to RWMol (exception)")]
    ConversionException(String),
}

impl RWMol {
    pub fn from_mol_block(
        mol_block: &str,
        sanitize: bool,
        remove_hs: bool,
        strict_parsing: bool,
    ) -> Option<Self> {
        let_cxx_string!(mol_block = mol_block);

        let ptr =
            rw_mol_ffi::rw_mol_from_mol_block(&mol_block, sanitize, remove_hs, strict_parsing);

        if ptr.is_null() {
            None
        } else {
            Some(RWMol { ptr })
        }
    }

    pub fn as_smiles(&self) -> String {
        let cast_ptr = unsafe {
            std::mem::transmute::<
                SharedPtr<rdkit_sys::rw_mol_ffi::RWMol>,
                SharedPtr<rdkit_sys::ro_mol_ffi::ROMol>,
            >(self.ptr.clone())
        };
        ro_mol_ffi::mol_to_smiles(&cast_ptr)
    }

    pub fn to_ro_mol(self) -> ROMol {
        let ptr = unsafe {
            std::mem::transmute::<
                SharedPtr<rdkit_sys::rw_mol_ffi::RWMol>,
                SharedPtr<rdkit_sys::ro_mol_ffi::ROMol>,
            >(self.ptr)
        };
        ROMol { ptr }
    }

    pub fn from_smarts(smarts: &str) -> Result<Self, RWMolError> {
        let_cxx_string!(smarts = smarts);

        let ptr = rdkit_sys::rw_mol_ffi::smarts_to_mol(&smarts);
        match ptr {
            Ok(ptr) => {
                if ptr.is_null() {
                    Err(RWMolError::UnknownConversionError)
                } else {
                    Ok(RWMol { ptr })
                }
            }
            Err(e) => Err(RWMolError::ConversionException(e.to_string())),
        }
    }
}

impl Clone for RWMol {
    fn clone(&self) -> Self {
        let ptr = rw_mol_ffi::rw_mol_from_rw_mol(&self.ptr);
        RWMol { ptr }
    }
}

impl std::fmt::Debug for RWMol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let smiles = self.as_smiles();
        f.debug_tuple("RWMol").field(&smiles).finish()
    }
}
