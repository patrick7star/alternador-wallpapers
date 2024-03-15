
/** 
 BD para gravar em disco todas alterações já realizadas, como também para 
 ajudar na reparações de redundâncias feitas na seleção aleatória.
*/

// biblioteca padrão do Rust:
use std::fs::{OpenOptions, read_to_string, File};
use std::io::{Write};
use std::path::{PathBuf};
// usando própria biblioteca:
// use super::BD1;
use crate::compilacao::computa_caminho;
use crate::constantes::{SELECOES_FEITAS, Str};

pub fn grava_escolha(caminho:PathBuf) -> bool {
   let path_str = "data/ultima_escolha.txt";
   let caminho_bd = computa_caminho(path_str);
   // abrindo bd ...
   let mut arquivo:File = {
      OpenOptions::new()
      .create(true)
      .write(true)
      // .open(BD1)
      .open(caminho_bd)
      .unwrap()
   };
   // todas modificações feitas.
   let nome:&str = {
      caminho
      .file_name()
      .unwrap()
      .to_str()
      .unwrap()
   };
   let diretorio:&str = {
      caminho.as_path()
      .parent().unwrap()
      .to_str().unwrap()
   };
   // primeiro o caminho até o arquivo.
   arquivo.write(diretorio.as_bytes()).unwrap();
   // quebra-de-linha necessária para separação.
   arquivo.write(b"\n").unwrap();
   // agora o nome do arquivo em sí.
   arquivo.write(nome.as_bytes()).unwrap();
   // quebra-de-linha necessária para separação.
   arquivo.write(b"\n").unwrap();

   //faz agora a escrita da seleção feita.
   registra_no_historico(nome);

   // confirmação de tudo okay ...
   return true;
}

pub fn le_escolha() -> Result<PathBuf, Str> {
   // lendo todo arquivo, e colocando num interador
   // baseado nas quebra-de-linhas.
   let conteudo:String = {
      let path_str = "data/ultima_escolha.txt";
      let caminho_bd = computa_caminho(path_str);
      // let pth = Path::new(BD1);
      match read_to_string(caminho_bd) {
         Ok(resultado) => resultado,
         Err(_) => { return Err("arquivo foi apagado!"); }
      }
   };
   let mut conteudo = conteudo.lines();
   let diretorio = conteudo.next().unwrap();
   let nome = conteudo.next().unwrap();
   // formando 'Caminho(Path)' ...
   let mut caminho:PathBuf = PathBuf::new();
   caminho.push(diretorio);
   caminho.push(nome);
   // retornando o que foi obtido.
   return Ok(caminho);
}

/* grava seleção feito num banco de dados próprio, para propósitos de 
 * pesquisas posteriores. */
fn registra_no_historico(nome: &str) {
   if cfg! (debug_assertions)
      { println!("caminho(SELEÇÔES FEITAS): {}", SELECOES_FEITAS); }

   let path_str: &str = "data/historico_de_escolhas_feitas.txt";
   let caminho_historico = computa_caminho(path_str);
   // abrindo bd ...
   let mut arquivo: File = {
      OpenOptions::new()
      .create(true)
      .append(true)
      // .open(SELECOES_FEITAS)
      .open(caminho_historico)
      .unwrap()
   };

   arquivo.write(nome.as_bytes()).unwrap();
   /* colocando quebra de linha, mirando 
    * próxima concatenação. */
   arquivo.write("\n".as_bytes()).unwrap();
}

#[cfg(test)]
mod tests {
   use super::*;
   use crate::transicao::parte_i;
   use std::collections::HashSet;
   use std::fs::{remove_file, read_to_string};
   use std::path::Path;

   #[test]
   fn testando_escrita() {
      let caminho = "dir/sub_dir/sub_sub_dir/arquivo.txt";
      let arquivo_teste = Path::new(caminho).to_path_buf();
      assert!(grava_escolha(arquivo_teste.clone()));
   }

   #[test]
   fn testa_leitura_dos_dados() {
      let caminho = le_escolha().unwrap();
      let mut caminho_i = PathBuf::new();
      caminho_i.push("dir");
      caminho_i.push("sub_dir");
      caminho_i.push("sub_sub_dir");
      caminho_i.push("arquivo.txt");
      assert_eq!(caminho, caminho_i);
   }

   #[test]
   fn verificacao_escrita_de_selecao() {
      /* teste, inicialmente, só funciona
       * quando não existe um arquivo, para
       * não comrromper os dados já depositados
       * lá. */
      let caminho = Path::new(SELECOES_FEITAS);
      let mensagem = concat!(
         "arquivo já existe, pode comrromper ", 
         "atual banco de dados"
      );
      if caminho.exists()
         { panic!("{}", mensagem); }
      let mut nomes = HashSet::<String>::new();

      for pth in parte_i() { 
         let nome = {
            pth.to_str().unwrap()
            .rsplit_once("/").take()
            .unwrap().1
         };
         // colocando nome no histórico.
         registra_no_historico(nome);
         println!("{}", nome); 
         nomes.insert(nome.to_string());
      }
      // colocou todos lá.
      assert!(!nomes.is_empty());

      /* pegando todas strings escritas no arquivo,
       * e as enfiando numa array. */
      let mut conteudo: Vec<String> = {
         read_to_string(SELECOES_FEITAS)
         .unwrap().split_whitespace()
         .map(|item| item.to_string())
         .collect()
      };
      let nomes_no_historico: HashSet<String>;
      nomes_no_historico = HashSet::from_iter(conteudo.drain(..));
      assert!(!nomes_no_historico.is_empty());
      println!("{:?}", nomes_no_historico);
      /* ambos conjutos têm que ser iguais, ou seja,
       * conter os mesmos elementos. */
      assert_eq!(nomes_no_historico, nomes);

      // excluindo arquivo para nova rodada.
      remove_file(SELECOES_FEITAS).unwrap();
   }
}
