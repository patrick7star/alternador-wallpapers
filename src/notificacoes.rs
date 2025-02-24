// Componente do próprio projeto:
use crate::banco_de_dados::le_escolha;
use crate::transicao::duracao_atual_transicao;
use crate::constantes::ULTIMA_NOTIFICACAO;
// Bibliotecas externas:
use utilitarios::legivel::tempo as Tempo;
// Biblioteca padrão do Rust:
use std::path::Path;
use std::process::Command;
use std::fs::{read, write};


// extensão para o objeto String.
trait Extensao {
   fn titulo(&self) -> String;
}
 
impl Extensao for String {
   // Faz uma string como título, uma palavra "capitalizada".
   fn titulo(&self) -> String {
      let mut nova_str = String::new();
      for s in self.split(' ') {
         let primeiro_char = s.get(0..1).unwrap();
         nova_str += primeiro_char.to_uppercase().as_str();
         nova_str += s.get(1..).unwrap();
         nova_str.push(' ');
      }
      return nova_str;
   }
}

/* retorna atual transição, removendo a parte importante, no caso o nome 
 dela, também há processamento forma que tal nome é apresentado. Aplica a 
 técnica palavras capitalizadas(se houver mais que uma). */
fn atual_transicao() -> String {
   let caminho_contido = le_escolha().unwrap();

   /* retirando o traço por espaço e colocando tudo maiúscula. */
   caminho_contido.as_path()
   .file_name().unwrap()
   .to_str().unwrap()
   .strip_suffix(".xml")
   .unwrap().replace(&"_", &" ")
   .titulo().trim_end()
   .to_string()
}


/* verifica se o wallpaper que plotou notificação anterior, é o mesmo que 
 está sendo plotado no momento. */
fn mesma_que_a_anterior(anterior: &str) -> bool {
   let caminho = Path::new(ULTIMA_NOTIFICACAO);

   /* se não existe ainda tal arquivo, é o mesmo
    * que não pode ser igual, obviamente. */
   if !caminho.exists() { return false; }

   let bytes = read(caminho).unwrap();
   let conteudo = String::from_utf8_lossy(&bytes[..]);

   // valor lógico da pergunta.
   conteudo == anterior
}


/* faz uma notificação da atual transição aplicada ao sistema. */
pub fn popup_notificacao_de_transicao() {
   /* obtêm o nome da atual transição, e trabalha um pouquinho nela. */
   let nome_transicao = atual_transicao();
   let tempo: &str;
   let mensagem: String;

   if mesma_que_a_anterior(nome_transicao.as_str()) {
      // halve previous time.
      tempo = "--expire-time=12500";
      mensagem = format!(
         "a atual transição continuará, aguarde mais {}",
         Tempo(duracao_atual_transicao().as_secs(), true)
      );
   } else {
      tempo = "--expire-time=25000";
      mensagem = format!(
         "a nova transição de imagem \"{}\" foi colocada",
         nome_transicao
      );
   }

   let argumentos: [&str; 4] = [
      //"--expire-time=25000",
      tempo, "--icon=object-rotate-left",
      "--app-name=AlternaWallpaper",
      mensagem.as_str()
   ];

   // executando comando ...
   Command::new("notify-send")
   .args(argumentos.into_iter())
   .spawn().unwrap()
   .wait().unwrap();

   if cfg!(debug_assertions) {
      // emitindo que a mensagem foi enviada.
      println!("Notificação foi \"plotada\" com sucesso.");
   }

   // registrando nova alteração.
   let bytes = nome_transicao.bytes().collect::<Vec<u8>>();
   write(ULTIMA_NOTIFICACAO, &bytes[..]).unwrap();
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
   use super::*;
   use std::fs::remove_file;
   // use utilitarios::legivel::tempo;

   #[test]
   fn atualTransicao() 
      { println!("{}", atual_transicao()); }

   #[test]
   fn simplesMesmaQueAAnterior() {
      let s = dbg!(atual_transicao());
      assert!(mesma_que_a_anterior(s.as_str()));
      let s = "Desenhos Wallpapers";
      assert!(!mesma_que_a_anterior(s));
   }

   #[test]
   fn simplesPopupNotificacao() 
      { popup_notificacao_de_transicao(); }

   use std::fs::{rename, copy};
   use std::env::{set_var, var};
   use std::ffi::OsStr;
   use std::str::FromStr;

   /* clona arquivo, então renomea o original, para que não sofra alteração      caso ele seja importante para outras coisas. Quando chamada novamente,
     restauro o original, e deleta o clone.  
   */
   fn salva_e_restaura<P>(mut caminhos: Vec<&P>)
     where P: AsRef<Path> + ?Sized + ToString
   {
      // nome das variáveis de ambientes.
      let nome_variavel = OsStr::new("GUARDADO");
      let total_variavel = OsStr::new("TOTAL_GUARDADOS");
      // parse de string para valor booleano.
      let valor_logico = match var(nome_variavel) {
         Ok(s) => bool::from_str(s.as_str()).unwrap(),
         Err(_) => false
      };

      if !valor_logico {
         // gravando o total de arquivos:
         set_var(total_variavel, caminhos.len().to_string());
         // realização para cada caminho dado.
         for caminho in caminhos.drain(..) {
            /* se não estiver guardado, então à variável,
             * possivelmente, ainda nem existe, então será
             * feito o processo, então, ela será criada para
             * confirmar que tais instruções foram realizadas. */
            let novo_caminho = caminho.to_string()+ &".copia";
            let novo_nome = Path::new(novo_caminho.as_str());
            rename(caminho, novo_nome).unwrap();
            copy(novo_nome, caminho).unwrap();
         }
         // marca como gravado para toda lista.
         set_var(nome_variavel, OsStr::new("true"));
      } else {
         if let Ok(t) = var(total_variavel) { 
            assert_eq!(
               usize::from_str(t.as_str()).unwrap(), 
               caminhos.len()
            );
            println!("qtd. de arquivos restaurados: {}", t); 
         }
         for caminho in caminhos.drain(..) {
            /* caso tenha sido realizado anteriormente,
             * agora fica o processo de restauração, ou
             * seja, ele excluirá o arquivo utilizado,
             * e trocará o 'backup' pelo antigo nome,
             * este que ficou alí inalterado. */
            let novo_caminho = caminho.to_string()+ &".copia";
            remove_file(caminho).unwrap();
            let nome_atual = Path::new(novo_caminho.as_str());
            rename(nome_atual, caminho).unwrap();
         }
         set_var(nome_variavel, "false");
      }
   }

   #[test]
   fn mesmaQueAAnterior() {
      // faz backup.
      salva_e_restaura(vec![ULTIMA_NOTIFICACAO]);
      let mut entradas = vec![
         "Wallpapers De Flores", "Porta Wallpapers",
         "Porta Wallpapers", "Wallpapers De Patos",
         "Wallpapers De Flores", "Wallpapers De Flores",
         "Wallpapers De Flores", "Wallpapers De Patos",
         "Porta Wallpapers"
      ];
      let mut saidas = vec![
         false, false, true, false, false,
         true, true, false, false
      ];

      // feito na mão primeiro.
      println!("{}", atual_transicao());

      for (E, S) in entradas.drain(..).zip(saidas.drain(..)) {
         // resposta tem que ser esperada.
         assert_eq!(mesma_que_a_anterior(E), S); 
         println!("\u{2196} {} \u{27fc}{:>8}", E, S);

         // escrevendo "atual seleção".
         let bytes = E.bytes().collect::<Vec<u8>>();
         let slice = bytes.as_slice();
         write(ULTIMA_NOTIFICACAO, slice).unwrap();
      }
      // restaura original.
      salva_e_restaura(vec![ULTIMA_NOTIFICACAO]);
   }
}
