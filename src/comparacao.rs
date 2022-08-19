
/* Submódulo do atualizações, onde será
 * realizado as contagens de imagens em
 * cada diretório válido; porém tal contagem
 * tem que bater com a contagem nos arquivo
 * XML gerado, para que dê um sinal se é 
 * hora de atualizar ou não tal arquivo de
 * algum diretório.
 */

// biblioteca padrão:
use std::process::{Command, Output};
use std::path::Path;
use std::ffi::OsStr;
use std::str::FromStr;

// próprio caixote:
use super::transicao;
use crate::PYTHON;


// conta quantia de imagens de um diretório.
fn conta_imagens(caminho:&Path) -> u8 {
   let mut qtd = 0;
   let mut nao_tem_xml: bool = true;

   // verifica se há algum arquivo XML.
   for item in caminho.read_dir().unwrap() {
      let caminho_i = item.unwrap().path();
      let extensao = {
         caminho_i
         .as_path()
         .extension()
         .unwrap()
      };
      if extensao == OsStr::new("xml")
         { nao_tem_xml = false; }
      // contando iterações.
      qtd += 1;
   }

   // descontabilizada arquivo XML na contagem.
   if nao_tem_xml
      { qtd as u8 }
   else
      { (qtd as u8) - 1 }
}

// conta o total de imagens, lendo o arquivo XML.
fn conta_imagens_xml(caminho:&Path) -> u8 {
   let caminho_str = caminho.to_str().unwrap();
   // mudando de diretório.
   let dir = concat!(
      env!("RUST_CODES"),
      "/alternador-wallpapers",
      "/extern_lib",
      "/slide_background"
   );
   let instrucao = format!(
      "{}; print({}('{path}'))",
      "import xml_info as XI",
      "XI.total_de_imagens", 
      path=caminho_str
   );
   let resultado:Result<Output, _> = {
      Command::new(PYTHON)
      .current_dir(Path::new(dir))
      .arg("-c")
      .arg(instrucao.as_str())
      .output()
   };
   // transforma numa String o vetor de bytes.
   let resultado = String::from_utf8(resultado.unwrap().stdout).unwrap();
   // total de bytes que sairam.
   let qb = resultado.len();
   // converte num inteiro positivo simples.
   return u8::from_str(&resultado.as_str()[..(qb-1)]).unwrap();
}

/* a resposta se todos os wallpapers existentes
 * no diretório, foram contabilizados nos 
 * arquivos XML's. Parte da suposição que estão,
 * caso contrário o valor será "falso".
 */
pub fn contabilidade_esta_ok() -> bool {
   // lista de todos XML's.
   let lista = transicao::parte_i();

   // verificando igualdade de um-a-um.
   for xml in lista.iter() {
      let caminho = xml.as_path();
      let caminho_dir = caminho.parent().unwrap();
      let a = conta_imagens_xml(caminho);
      let b = conta_imagens(caminho_dir);
      if a != b
         { return false; }
   }
   /* se chegar aqui, então todos no XML
    * batem com os contados no diretório. */
   return true;
}

/* informação sobre a razão contagem pelo
 * XML, e contagem acessando o diretório. */
pub fn razao_info() -> String {
   // lista de todos XML's
   let lista = transicao::parte_i();
   // contador de info-finais adicionadas.
   let mut contador: u8 = 0;
   // linhas de strings a concatenar.
   let mut texto = String::from("diretórios acrescidos:");

   // verificando igualdade de um-a-um.
   for xml in lista.iter() {
      let caminho = xml.as_path();
      let caminho_dir = caminho.parent().unwrap();
      let a = conta_imagens_xml(caminho);
      let b = conta_imagens(caminho_dir);

      // ignorar razão que não faz sentido.
      if a == 0 && b == 0
         { continue; }
      // mostrar apenas as desproporcionais.
      else if a != b {
         // nome da transição:
         let nome:&str = {
            caminho
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
         };
         let nome = nome.replace(".xml", "");
         let mensagem = format!(
            "\n\t{}({}/{})", 
            nome, a, b
         );
         texto.push_str(mensagem.as_str());
         contador += 1;
      } 
   }

   if contador == 0 {
      // limpando cabeçalho ...
      texto.clear();
      texto.push_str("todas imagens dos "); 
      let qtd_dirs = lista.len();
      texto.push_str(qtd_dirs.to_string().as_str());
      texto.push_str(" diretórios foram contabilizadas.")
   } 
   
   // concatena informações.
   return texto;
}
   

#[cfg(test)]
mod tests {
   use std::path::PathBuf;

   #[test]
   fn testa_conta_imagens() {
      let mut pth = PathBuf::new();
      pth.push(env!("HOME"));
      pth.push("Pictures");
      pth.push("computação");
      let qtd_i = dbg!(super::conta_imagens(pth.as_path()));
      assert!(qtd_i > 5);
   }

   #[test]
   fn contagens_iguais() {
      let caminho = super::Path::new(env!("HOME"))
      .join("Pictures/aves/aves.xml");
      // cotabilização pelo XML.
      let a = super::conta_imagens_xml(&caminho);
      // cotabilização pelo diretório.
      let b = super::conta_imagens(caminho.parent().unwrap());
      // tem que ser iguais.
      assert_eq!(a, b);
   }

   #[test]
   fn todas_possibilidades_ok() {
      let lista = super::transicao::parte_i();
      for item in lista.iter() {
         let caminho = dbg!(item.as_path());
         // cotabilização pelo XML.
         let a = super::conta_imagens_xml(&caminho);
         // cotabilização pelo diretório.
         let b = super::conta_imagens(caminho.parent().unwrap());
         // tem que ser iguais.
         assert_eq!(a, b);
      }
   }
}
