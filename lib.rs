#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod contatos {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;

    #[derive(scale::Encode, scale::Decode, Clone, Debug, PartialEq, Default)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub enum Categoria {
        Amigo,
        Familiar,
        #[default]
        Colega,
    }

    #[derive(scale::Encode, scale::Decode, Clone, Debug, PartialEq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct Contato {
        nome: String,
        telefone: String,
        idade: u32,
        data_aniversario: String,
        categoria: Categoria,
    }

    #[ink(storage)]
    pub struct Contatos {
        contatos: Mapping<u32, Contato>,
        next_id: u32,
    }

    impl Default for Contatos {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Contatos {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                contatos: Mapping::default(),
                next_id: 0,
            }
        }

        #[ink(message)]
        pub fn criar_contato(
            &mut self,
            nome: String,
            telefone: String,
            idade: u32,
            data_aniversario: String,
            categoria: Categoria,
        ) -> u32 {
            let id = self.next_id;
            let contato = Contato {
                nome,
                telefone,
                idade,
                data_aniversario,
                categoria,
            };
            self.contatos.insert(id, &contato);
            self.next_id = self.next_id.checked_add(1).expect("Overflow when incrementing next_id");
            id
        }

        #[ink(message)]
        pub fn ler_contato(&self, id: u32) -> Option<Contato> {
            self.contatos.get(id)
        }

        #[ink(message)]
        pub fn atualizar_contato(
            &mut self,
            id: u32,
            nome: String,
            telefone: String,
            idade: u32,
            data_aniversario: String,
            categoria: Categoria,
        ) -> bool {
            if let Some(mut contato) = self.contatos.get(id) {
                contato.nome = nome;
                contato.telefone = telefone;
                contato.idade = idade;
                contato.data_aniversario = data_aniversario;
                contato.categoria = categoria;
                self.contatos.insert(id, &contato);
                true
            } else {
                false
            }
        }

        #[ink(message)]
        pub fn deletar_contato(&mut self, id: u32) -> bool {
            if self.contatos.contains(id) {
                self.contatos.remove(id);
                true
            } else {
                false
            }
        }

        #[ink(message)]
        pub fn listar_contatos(&self) -> Vec<Contato> {
            let mut lista = Vec::new();
            for id in 0..self.next_id {
                if let Some(contato) = self.contatos.get(id) {
                    lista.push(contato);
                }
            }
            lista
        }
    }
}