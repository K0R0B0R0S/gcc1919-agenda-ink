#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod agenda {
    use ink::prelude::string::ToString;
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;

    // ----- Contatos -----

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
        pub nome: String,
        pub telefone: String,
        pub idade: u32,
        pub data_aniversario: String,
        pub categoria: Categoria,
    }

    // ----- Compromissos -----

    #[derive(scale::Encode, scale::Decode, Clone, Debug, PartialEq, Default)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub enum Prioridade {
        Alta,
        Media,
        #[default]
        Baixa,
    }

    #[derive(scale::Encode, scale::Decode, Clone, Debug, PartialEq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct Compromisso {
        pub titulo: String,
        pub data: String,
        pub hora: String,
        pub prioridade: Prioridade,
        pub duracao: i32,
    }

    #[ink(storage)]
    pub struct Agenda {
        contatos: Mapping<u32, Contato>,
        compromissos: Mapping<u32, Compromisso>,
        next_contato_id: u32,
        next_compromisso_id: u32,
    }

    impl Default for Agenda {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Agenda {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                contatos: Mapping::default(),
                compromissos: Mapping::default(),
                next_contato_id: 0,
                next_compromisso_id: 0,
            }
        }

                // ----- Validações de Inputs -----

                fn validar_data(data: &str) -> bool {
                    let partes: Vec<&str> = data.split('/').collect();
                    if partes.len() != 3 {
                        return false;
                    }
                
                    let dia: u32 = partes[0].parse().unwrap_or(0);
                    let mes: u32 = partes[1].parse().unwrap_or(0);
                    let ano: u32 = partes[2].parse().unwrap_or(0);
                
                    // Validar se a data está no formato dd/mm/aaaa
                    if dia == 0 || mes == 0 || ano == 0 || mes > 12 || dia > 31 {
                        return false;
                    }
                
                    // Validação simples para dias do mês
                    match mes {
                        4 | 6 | 9 | 11 => dia <= 30,
                        2 => {
                            if ano % 4 == 0 && (ano % 100 != 0 || ano % 400 == 0) {
                                dia <= 29 // ano bissexto
                            } else {
                                dia <= 28
                            }
                        }
                        _ => dia <= 31,
                    }
                }                
        
                fn validar_hora(hora: &str) -> bool {
                    let partes: Vec<&str> = hora.split(':').collect();
                    if partes.len() != 2 {
                        return false;
                    }
        
                    let hora: u32 = partes[0].parse().unwrap_or(0);
                    let minuto: u32 = partes[1].parse().unwrap_or(0);
        
                    // Validar se a hora está no formato hh:mm
                    hora < 24 && minuto < 60
                }

        // ----- Métodos para Contatos -----

        /// Cria um novo contato na agenda.
        #[ink(message)]
        pub fn criar_contato(
            &mut self,
            nome: String,
            telefone: String,
            idade: u32,
            data_aniversario: String,
            categoria: Categoria,
        ) -> Result<u32, String> {
            if nome.is_empty() {
                return Err("Nome não pode estar vazio".to_string());
            }
        
            if telefone.is_empty() {
                return Err("Telefone não pode estar vazio".to_string());
            }
        
            if !Self::validar_data(&data_aniversario) {
                return Err("Data de aniversário inválida. O formato deve ser dd/mm/aaaa.".to_string());
            }
            let id = self.next_contato_id;
            let contato = Contato {
                nome,
                telefone,
                idade,
                data_aniversario,
                categoria,
            };
            self.next_contato_id = self.next_contato_id.checked_add(1).expect("Overflow");
            self.contatos.insert(id, &contato);
            Ok(id)
        }

        /// Lê um contato da agenda.
        #[ink(message)]
        pub fn ler_contato(&self, id: u32) -> Option<Contato> {
            self.contatos.get(id)
        }

        /// Atualiza um contato da agenda.
        #[ink(message)]
        pub fn atualizar_contato(
            &mut self,
            id: u32,
            nome: String,
            telefone: String,
            idade: u32,
            data_aniversario: String,
            categoria: Categoria,
        ) -> Result<bool, String> {
            if nome.is_empty() {
                return Err("Nome não pode estar vazio".to_string());
            }
        
            if telefone.is_empty() {
                return Err("Telefone não pode estar vazio".to_string());
            }
        
            if !Self::validar_data(&data_aniversario) {
                return Err("Data de aniversário inválida. O formato deve ser dd/mm/aaaa.".to_string());
            }
        
            if let Some(mut contato) = self.contatos.get(id) {
                contato.nome = nome;
                contato.telefone = telefone;
                contato.idade = idade;
                contato.data_aniversario = data_aniversario;
                contato.categoria = categoria;
                self.contatos.insert(id, &contato);
                Ok(true)
            } else {
                Err("Contato não encontrado".to_string())
            }
        }
        

        /// Deleta um contato da agenda.
        #[ink(message)]
        pub fn deletar_contato(&mut self, id: u32) -> bool {
            if self.contatos.contains(id) {
                self.contatos.remove(id);
                true
            } else {
                false
            }
        }

        /// Lista todos os contatos da agenda.
        #[ink(message)]
        pub fn listar_contatos(&self) -> Vec<Contato> {
            let mut lista = Vec::new();
            for id in 0..self.next_contato_id {
                if let Some(contato) = self.contatos.get(id) {
                    lista.push(contato);
                }
            }
            lista
        }

        // ----- Métodos para Compromissos -----

        /// Cria um novo compromisso na agenda.
        #[ink(message)]
        pub fn criar_compromisso(
            &mut self,
            titulo: String,
            data: String,
            hora: String,
            prioridade: Prioridade,
            duracao: i32,
        ) -> Result<u32, String> {
            if titulo.is_empty() {
                return Err("Título não pode estar vazio".to_string());
            }
        
            if !Self::validar_data(&data) {
                return Err("Data inválida. O formato deve ser dd/mm/aaaa.".to_string());
            }
        
            if !Self::validar_hora(&hora) {
                return Err("Hora inválida. O formato deve ser hh:mm.".to_string());
            }
            
            let id = self.next_compromisso_id;
            let compromisso = Compromisso {
                titulo,
                data,
                hora,
                prioridade,
                duracao,
            };
            self.next_compromisso_id = self.next_compromisso_id.checked_add(1).expect("Overflow");
            self.compromissos.insert(id, &compromisso);
            Ok(id)
        }

        /// Lê um compromisso da agenda.
        #[ink(message)]
        pub fn ler_compromisso(&self, id: u32) -> Option<Compromisso> {
            self.compromissos.get(id)
        }

        /// Atualiza um compromisso da agenda.
        #[ink(message)]
        pub fn atualizar_compromisso(
            &mut self,
            id: u32,
            titulo: String,
            data: String,
            hora: String,
            prioridade: Prioridade,
            duracao: i32,
        ) -> Result<bool, String> {
            if titulo.is_empty() {
                return Err("Título não pode estar vazio".to_string());
            }
        
            if !Self::validar_data(&data) {
                return Err("Data inválida. O formato deve ser dd/mm/aaaa.".to_string());
            }
        
            if !Self::validar_hora(&hora) {
                return Err("Hora inválida. O formato deve ser hh:mm.".to_string());
            }
        
            if let Some(mut compromisso) = self.compromissos.get(id) {
                compromisso.titulo = titulo;
                compromisso.data = data;
                compromisso.hora = hora;
                compromisso.prioridade = prioridade;
                compromisso.duracao = duracao;
                self.compromissos.insert(id, &compromisso);
                Ok(true)
            } else {
                Err("Compromisso não encontrado".to_string())
            }
        }
        
        /// Deleta um compromisso da agenda.
        #[ink(message)]
        pub fn deletar_compromisso(&mut self, id: u32) -> bool {
            if self.compromissos.contains(id) {
                self.compromissos.remove(id);
                true
            } else {
                false
            }
        }

        /// Lista todos os compromissos da agenda.
        #[ink(message)]
        pub fn listar_compromissos(&self) -> Vec<Compromisso> {
            let mut lista = Vec::new();
            for id in 0..self.next_compromisso_id {
                if let Some(compromisso) = self.compromissos.get(id) {
                    lista.push(compromisso);
                }
            }
            lista
        }
    }
}

#[cfg(test)]
mod tests {
    use super::agenda::*;

    #[ink::test]
    fn test_criar_contato() {
        let mut agenda = Agenda::new();

        // Teste criando um contato válido
        let nome = "John Doe".to_string();
        let telefone = "123456789".to_string();
        let idade = 30;
        let data_aniversario = "01/01/1990".to_string();
        let categoria = Categoria::Amigo;

        let result = agenda.criar_contato(nome.clone(), telefone.clone(), idade, data_aniversario.clone(), categoria.clone());
        assert!(result.is_ok(), "Falha ao criar contato");

        let id = result.unwrap();
        let contato = agenda.ler_contato(id).expect("O contato deve existir");

        assert_eq!(contato.nome, nome);
        assert_eq!(contato.telefone, telefone);
        assert_eq!(contato.idade, idade);
        assert_eq!(contato.data_aniversario, data_aniversario);
        assert_eq!(contato.categoria, categoria);
    }

    #[ink::test]
    fn test_criar_contato_data_invalida() {
        let mut agenda = Agenda::new();

        // Teste criando um contato com data inválida
        let nome = "John Doe".to_string();
        let telefone = "123456789".to_string();
        let idade = 30;
        let data_aniversario = "32/13/1990".to_string(); // Data inválida
        let categoria = Categoria::Amigo;

        let result = agenda.criar_contato(nome, telefone, idade, data_aniversario, categoria);
        assert!(result.is_err(), "Contato não deve ser criado com data inválida");
    }

    #[ink::test]
    fn test_atualizar_contato() {
        let mut agenda = Agenda::new();

        // Cria um contato válido
        let nome = "John Doe".to_string();
        let telefone = "123456789".to_string();
        let idade = 30;
        let data_aniversario = "01/01/1990".to_string();
        let categoria = Categoria::Amigo;

        let id = agenda.criar_contato(nome.clone(), telefone.clone(), idade, data_aniversario.clone(), categoria).unwrap();

        // Atualiza o contato com novas informações
        let new_nome = "Jane Doe".to_string();
        let new_telefone = "987654321".to_string();
        let new_idade = 31;
        let new_data_aniversario = "02/02/1990".to_string();
        let new_categoria = Categoria::Familiar;

        let update_result = agenda.atualizar_contato(id, new_nome.clone(), new_telefone.clone(), new_idade, new_data_aniversario.clone(), new_categoria.clone());
        assert!(update_result.is_ok(), "Falha ao atualizar contato");

        let updated_contato = agenda.ler_contato(id).expect("O contato deve existir");
        assert_eq!(updated_contato.nome, new_nome);
        assert_eq!(updated_contato.telefone, new_telefone);
        assert_eq!(updated_contato.idade, new_idade);
        assert_eq!(updated_contato.data_aniversario, new_data_aniversario);
        assert_eq!(updated_contato.categoria, new_categoria);
    }

    #[ink::test]
    fn test_atualizar_contato_data_invalida() {
        let mut agenda = Agenda::new();

        // Cria um contato válido
        let nome = "John Doe".to_string();
        let telefone = "123456789".to_string();
        let idade = 30;
        let data_aniversario = "01/01/1990".to_string();
        let categoria = Categoria::Amigo;

        let id = agenda.criar_contato(nome, telefone, idade, data_aniversario, categoria).unwrap();

        // Tenta atualizar com data inválida
        let new_data_aniversario = "32/13/1990".to_string(); // Data inválida
        let update_result = agenda.atualizar_contato(id, "Jane Doe".to_string(), "987654321".to_string(), 31, new_data_aniversario, Categoria::Familiar);

        assert!(update_result.is_err(), "Contato não deve ser atualizado com data inválida");
    }

    #[ink::test]
    fn test_criar_compromisso() {
        let mut agenda = Agenda::new();

        // Teste criando um compromisso válido
        let titulo = "Reunião".to_string();
        let data = "01/01/2025".to_string();
        let hora = "14:00".to_string();
        let prioridade = Prioridade::Alta;
        let duracao = 60;

        let result = agenda.criar_compromisso(titulo.clone(), data.clone(), hora.clone(), prioridade.clone(), duracao);
        assert!(result.is_ok(), "Falha ao criar compromisso");

        let id = result.unwrap();
        let compromisso = agenda.ler_compromisso(id).expect("O compromisso deve existir");

        assert_eq!(compromisso.titulo, titulo);
        assert_eq!(compromisso.data, data);
        assert_eq!(compromisso.hora, hora);
        assert_eq!(compromisso.prioridade, prioridade);
        assert_eq!(compromisso.duracao, duracao);
    }

    #[ink::test]
    fn test_criar_compromisso_data_invalida() {
        let mut agenda = Agenda::new();

        // Teste criando um compromisso com data inválida
        let titulo = "Reunião".to_string();
        let data = "32/13/2025".to_string(); // Data inválida
        let hora = "14:00".to_string();
        let prioridade = Prioridade::Alta;
        let duracao = 60;

        let result = agenda.criar_compromisso(titulo, data, hora, prioridade, duracao);
        assert!(result.is_err(), "Compromisso não deve ser criado com data inválida");
    }

    #[ink::test]
    fn test_atualizar_compromisso() {
        let mut agenda = Agenda::new();

        // Cria um compromisso válido
        let titulo = "Reunião".to_string();
        let data = "01/01/2025".to_string();
        let hora = "14:00".to_string();
        let prioridade = Prioridade::Alta;
        let duracao = 60;

        let id = agenda.criar_compromisso(titulo.clone(), data.clone(), hora.clone(), prioridade, duracao).unwrap();

        // Atualiza o compromisso com novas informações
        let new_titulo = "Conferência".to_string();
        let new_data = "02/01/2025".to_string();
        let new_hora = "10:00".to_string();
        let new_prioridade = Prioridade::Media;
        let new_duracao = 90;

        let update_result = agenda.atualizar_compromisso(id, new_titulo.clone(), new_data.clone(), new_hora.clone(), new_prioridade.clone(), new_duracao);
        assert!(update_result.is_ok(), "Falha ao atualizar compromisso");

        let updated_compromisso = agenda.ler_compromisso(id).expect("O compromisso deve existir");
        assert_eq!(updated_compromisso.titulo, new_titulo);
        assert_eq!(updated_compromisso.data, new_data);
        assert_eq!(updated_compromisso.hora, new_hora);
        assert_eq!(updated_compromisso.prioridade, new_prioridade);
        assert_eq!(updated_compromisso.duracao, new_duracao);
    }

    #[ink::test]
    fn test_deletar_contato() {
        let mut agenda = Agenda::new();

        // Cria um contato válido
        let nome = "John Doe".to_string();
        let telefone = "123456789".to_string();
        let idade = 30;
        let data_aniversario = "01/01/1990".to_string();
        let categoria = Categoria::Amigo;

        let id = agenda.criar_contato(nome, telefone, idade, data_aniversario, categoria).unwrap();

        // Deleta o contato
        let delete_result = agenda.deletar_contato(id);
        assert!(delete_result, "O contato deve ser deletado");

        // Garante que o contato não existe mais
        let deleted_contato = agenda.ler_contato(id);
        assert!(deleted_contato.is_none(), "O contato não deve existir após a exclusão");
    }

    #[ink::test]
    fn test_deletar_compromisso() {
        let mut agenda = Agenda::new();

        // Cria um compromisso válido
        let titulo = "Reunião".to_string();
        let data = "01/01/2025".to_string();
        let hora = "14:00".to_string();
        let prioridade = Prioridade::Alta;
        let duracao = 60;

        let id = agenda.criar_compromisso(titulo, data, hora, prioridade, duracao).unwrap();

        // Deleta o compromisso
        let delete_result = agenda.deletar_compromisso(id);
        assert!(delete_result, "O compromisso deve ser deletado");

        // Garante que o compromisso não existe mais
        let deleted_compromisso = agenda.ler_compromisso(id);
        assert!(deleted_compromisso.is_none(), "O compromisso não deve existir após a exclusão");
    }
}
