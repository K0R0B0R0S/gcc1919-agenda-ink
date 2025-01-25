#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod gcc1919_agenda {
    use ink::prelude::vec::Vec;
    use ink::prelude::string::String;
    use ink::storage::Mapping;


    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    #[derive(Default)]
    pub enum Categoria {
        Amigo,
        Familiar,
        #[default]
        Colega,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    #[derive(Default)]
    pub enum Prioridade {
        Alta,
        Media,
        #[default]
        Baixa,
    }

    #[derive(Debug, Clone, scale::Encode, scale::Decode, Default)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct Contato {
        nome: String,
        telefone: String,
        email: String,
        idade: u32,
        data_aniversario: String,
        categoria: Categoria,
    }

    #[derive(Debug, Clone, scale::Encode, scale::Decode, Default)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct Compromisso {
        titulo: String,
        data: String,
        hora: String,
        descricao: String,
        prioridade: Prioridade,
    }

    #[ink(storage)]
    pub struct GCC1919Agenda {
        contatos: Mapping<AccountId, Contato>,
        compromissos: Mapping<AccountId, Compromisso>,
        contato_keys: Vec<AccountId>,
        compromisso_keys: Vec<AccountId>,
    }

    impl Default for GCC1919Agenda {
        fn default() -> Self {
            Self::new()
        }
    }

    impl GCC1919Agenda {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                contatos: Mapping::new(),
                compromissos: Mapping::new(),
                contato_keys: Vec::new(),
                compromisso_keys: Vec::new(),
            }
        }

        #[ink(message)]
        pub fn adicionar_contato(&mut self, nome: String, telefone: String, email: String, idade: u32, data_aniversario: String, categoria: Categoria) {
            let caller = self.env().caller();
            let contato = Contato {
                nome,
                telefone,
                email,
                idade,
                data_aniversario,
                categoria,
            };
            self.contatos.insert(caller, &contato);
        }

        #[ink(message)]
        pub fn listar_contatos(&self) -> Vec<Contato> {
            // Use contato_keys to get all contacts stored in the Mapping
            self.contato_keys
                .iter()
                .filter_map(|key| self.contatos.get(key))  // Retrieve the contact by key
                .collect()  // Collect them into a Vec
}

        #[ink(message)]
        pub fn adicionar_compromisso(&mut self, titulo: String, data: String, hora: String, descricao: String, prioridade: Prioridade) {
            let caller = self.env().caller();
            let compromisso = Compromisso {
                titulo,
                data,
                hora,
                descricao,
                prioridade,
            };
            self.compromissos.insert(caller, &compromisso);
        }

        #[ink(message)]
        pub fn listar_compromissos(&self) -> Vec<Compromisso> {
            self.compromisso_keys
                .iter()
                .filter_map(|key| self.compromissos.get(key))
                .collect()
        }
    }
}