

/*!
 Onde serão pega todas configurações do programa, mexido nela a execução
 do programa se derá em tempo diferente. O arquivo será em JSON, e 
 conterá os seguintes tipos de configurações: datas especiais, tempo 
 inicial até a primeira transição, descartar alguns dias especiais,
 e etc.
*/

use serde_json::{self, Value};
use std::fs::{read_to_string};
use std::path::{Path, PathBuf};
use crate::constantes::ARQUIVO_CONF;

/* carrega o único arquivo de configuração JSON com todos valores 
 * necessários para execução do programa.
 */
fn carrega_configuracoes() -> Value {
   /* lê todo arquivo contendo a estrutura JSON. */
   match read_to_string(Path::new(ARQUIVO_CONF)) {
      Ok(conteudo) => {
         match serde_json::from_str(conteudo.as_str()) {
            Ok(valor) => valor,
            Err(_) => 
               { panic!("arquivo está com algum problema de sintaxe!"); }
         }
      } Err (_) =>
         { panic!("erro ao carregar configurações, ajuste-a!"); }
   }
}

use std::env::var;
/// atual raiz onde os principais wallpapers do sistema estão agora.
pub fn raiz_wallpapers() -> PathBuf {
   // tenta obter valor desta chave específica na configuração JSON.
   match carrega_configuracoes().get("raiz") {
      Some(variante) => {
         match variante {
            Value::String(conteudo) => { 
               let mut caminho = PathBuf::new();

               // expande variaveis de ambiente do caminho.
               for componente in conteudo.split('/') {
                  let chave = {
                     componente
                     .strip_prefix('$')
                     .unwrap_or(componente) 
                  };

                  if componente.contains("$")
                     { caminho.push(var(chave).unwrap()); }
                  else
                     { caminho.push(componente); }
               }

               caminho
            } _ => 
               { panic!("nenhuma chave string no JSON!"); }
         }
      } None =>
         { panic!("não existe tal 'raiz' no JSON."); }
   }
}

/** caminhos externos à raiz, provavelmente espalhado por todo sistema
 de arquivos do sistema. */
pub fn wallpapers_externos() -> Vec<PathBuf> {
   let mut caminhos = Vec::<PathBuf>::new();
   let config = carrega_configuracoes();
   let nulo = serde_json::json!(Vec::<PathBuf>::new());
   let chave: &str = "wallpapers-externos";
   let array = config.get(chave).unwrap_or(&nulo);
   let mut posicao = 0;

   while let Some(dado) = array.get(posicao) {
      match dado {
         Value::String(s) => { 
            let caminho = Path::new(s);
            caminhos.push(caminho.to_path_buf()); 
         } _ => 
            { panic!("tipo de dado dissonante na array."); }
      };
      // tentando próximo índice.
      posicao += 1;
   }

   caminhos
}

// tipos tratados abaixo:
type LinhaData = (String, DateTuple, DateTuple);
type DEs = Option<Vec<(String, DateTuple, DateTuple)>>;

use date_time::date_tuple::DateTuple;
use std::str::FromStr;


fn extrai_data(string: &str) -> DateTuple {
   let partes = string.split_once("/").unwrap();
   let mes = u8::from_str(partes.1.trim()).unwrap();
   let dia = u8::from_str(partes.0.trim()).unwrap();
   let atual = DateTuple::today();
   DateTuple::new(atual.get_year(), mes, dia).unwrap()
}

pub fn coleta_datas_especiais_ii () -> DEs  {
   /* Aprimoramento do colhimento das configurações, usa JSON, com todo 
    * ferramental de "parsing" já pronto, e muito mais veloz, para 
    * colher tais tipos. */
   let mut lista: Vec<LinhaData> = vec![];
   let chave: &'static str = "datas-especiais";

   if let Value::Object (todos_dados) = carrega_configuracoes() {
      if let Value::Object (dicionario) = &todos_dados[chave] { 
         for (key, value) in dicionario.iter() { 
            if let Value::Array(periodos) = value {
               let inicio = extrai_data ( 
                  match &periodos[0] {
                     Value::String (data) => data.as_str(),
                     _ => todo! ()
                  }
               );
               let final_ = extrai_data (
                  match &periodos[1] {
                     Value::String (data) => data.as_str(),
                     _ => todo! ()
                  }
               );
               let mut nome = key.to_string();
               nome.push_str(".xml");
               lista.push((nome, inicio, final_));
            }
         }
      }
   }
   if lista.is_empty() { None }
   else { Some (lista) }
}

#[cfg(test)]
mod tests {
   use serde_json::{Result};
   use super::*;

   #[test]
   fn testando_serdejson() -> Result<()> {
      let texto = r#" {
         "nome": "Ana Clara",
         "idade": 21,
         "datas": [21, 1, 3],
         "raiz": "$HOME/Pictures"
      }"#;

      let v: Value = serde_json::from_str(texto)?;
      println!("passou 'parsing'");

      println!("{:#?}", v);
      Ok(())
   }

   #[test]
   fn carregamento_da_configuracao_geral() {
      let v = carrega_configuracoes(); 
      println!("conteúdo da configuração:\n{:#?}", v);
   }

   #[test]
   fn iteracao_da_configuracao() {
      let database =  carrega_configuracoes(); 
      println!("{}", database.get("raiz").unwrap());
      assert!(database.get("raiz").unwrap().is_string());
      println!("{:#?}", database.get("datas-especiais").unwrap());
   }

   #[test]
   fn extrai_raiz_dos_wallpapers() {
      println!(
         "onde estão wallpapers: {}", 
         raiz_wallpapers().display()
      );
   }

   #[test]
   fn visualiza_extracao_dos_wallpapers_externos() {
      println!(
         "caminhos dos wallpapers externos:\n{:#?}",
         wallpapers_externos()
      );
   }

   #[test]
   fn extracao_das_des() {
      for tupla in coleta_datas_especiais_ii().into_iter()
         { println! ("{tupla:#?}"); }
   }
}
