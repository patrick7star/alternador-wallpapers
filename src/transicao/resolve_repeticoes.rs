
/* cuida da quarta parte das funções auxiliares.
 * Ela tem como papel cuidar de que a próxima
 * seleção não seja a mesma que já está ativa,
 * ou foi utilizada por último.
 */

// próprio módulo:
use super::datas_especiais::{ parteIII, e_periodo_de_ferias };
// próprio biblioteca:
use crate::banco_de_dados::{grava_escolha, le_escolha};
// use crate::compilacao::computa_caminho;
// biblioteca padrão do Rust:
// use std::fs::read_to_string;
use std::path::{PathBuf};
// biblioteca externa:
use date_time::date_tuple::DateTuple;


use crate::configuracao::coleta_datas_especiais_ii;
/* tentanto reduzir repetições seguidas na seleção aleatória. Aqui elas 
 * são escritas com a numeração romana maiúsculas, simplesmente para 
 * diferenciar-se do original que ele está substituindo, já que este, não 
 * será deixado( descontinuado) de uma vez só. */
#[allow(non_snake_case)]
pub fn parteIV(hoje:DateTuple) -> PathBuf {
   /* extraindo feriados do arquivo de configuração. */
   /*
   let caminho = computa_caminho("data/datas_especiais.conf");
   let conteudo = read_to_string(caminho).unwrap();
   let feriados = match coleta_datas_especiais(conteudo) {
      Some(array) => array,
      None => 
         { panic!("sem feriados no arquivo de configuração."); }
   }; */
   let feriados = coleta_datas_especiais_ii().unwrap();
   let mut nova_transicao = parteIII(hoje.clone());
   // o que foi selecionado anterior.
   match le_escolha() {
      Ok(selecao_anterior) => {
         // não pode ser igual para não causar eventos repetidos.
         while selecao_anterior == nova_transicao {
            // atual para caber na tela(ajuda na codificação).
            let f = Some(feriados.clone());
            if e_periodo_de_ferias(hoje.clone(), f) 
               { break; }
            //if periodos_de_excecoes { break; }
            nova_transicao = parteIII(hoje.clone());
         }
      },
      Err(erro) => 
         { println!("\nERROR:{} ... continuando mesmo assim", erro); }
   };
   // grava opção a retornar.
   grava_escolha(nova_transicao.clone());
   return nova_transicao;
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
   use super::*;
   // locais dos arquivos tratados.
   const HISTORICO: &str = "./data/historico_de_escolhas_feitas.txt";
   const ULTIMA_MOD: &str = "./data/ultima_escolha.txt";

   use std::fs::{rename, remove_file, copy};
   use std::env::{set_var, var};
   use std::ffi::OsStr;
   use std::str::FromStr;
   use std::path::Path;
   /* clona arquivo, então renomea o original,
    * para que não sofra alteração caso ele seja
    * importante para outras coisas. Quando chamada
    * novamente, restauro o original, e deleta
    * o clone.  */
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

   use std::thread;
   use std::time::{Duration};
   use std::fs::OpenOptions;
   use std::io::Write;
   #[test]
   fn SalvaERestaura() {
      let mut f = {
         OpenOptions::new()
         .create(true)
         .write(true)
         .open("teste.txt")
         .unwrap()
      };
      f.write(b"embutindo uma simples string\n").unwrap();
      thread::sleep(Duration::from_secs(7));
      drop(f);
      salva_e_restaura(vec!["./teste.txt"]);
      let mut f = {
         OpenOptions::new()
         .append(true)
         .open("teste.txt")
         .unwrap()
      };
      f.write(b"uma string a mais adicionada\n")
      .unwrap();
      thread::sleep(Duration::from_secs(13));
      salva_e_restaura(vec!["./teste.txt"]);
      thread::sleep(Duration::from_secs(8));
      remove_file("teste.txt").unwrap();
   }

   #[test]
   #[ignore="não cai bem com múltiplas-threads."]
   fn resolvendoRepeticoesSeguidas() {
      let mut Siii: Vec<String> = vec![];
      let mut Siv: Vec<String> = vec![];
      /* único dia, para não colidir com dias com
       * feriados, já que neste caso, repetições
       * seriam inevitáveis. */
      let dia_qualquer= DateTuple::new(1999, 06, 15).unwrap();
      salva_e_restaura(vec![ULTIMA_MOD, HISTORICO]);

      for _ in 1..500{
         // resultado do método antigo.
         Siii.push(
            parteIII(dia_qualquer.clone())
            .file_name().unwrap()
            .to_str().unwrap()
            .to_string()
         );
         // resultado do método novo.
         let selecao = parteIV(dia_qualquer.clone());
         println!("parteIV: {:#?}", selecao);
         Siv.push(
            selecao
            .file_name().unwrap()
            .to_str().unwrap()
            .to_string()
         );
      }
      //salva_e_restaura("./data/ultima_escolha.txt");
      salva_e_restaura(vec![ULTIMA_MOD, HISTORICO]);
      // verificando repetições seguidas método crú.
      let (mut k, t) = (0, Siii.len());
      let (mut qtd, mut Q) = (0, 0);
      while k < t-1 {
         if Siii[k] == Siii[k+1] 
            { qtd += 1; }
         if Siv[k] == Siv[k+1] 
            { Q += 1; }
         k += 1;
      }
      assert!(qtd > 0);
      assert!(dbg!(qtd) > dbg!(Q));
      println!("razão de repetições {}/{}", qtd,500);
      assert_eq!(Q, 0);
   }

   use crate::transicao::parte_iv;
   #[test] 
   #[ignore="precisa que tenha precisas configurações no arquivo"] 
   fn comparaSaidasDeCadaEmPeriodosEspeciais() {
      // poupando atuais arquivos de alterações.
      salva_e_restaura(vec![HISTORICO, ULTIMA_MOD]);
      let mut feriados = vec![
         (OsStr::new("natal.xml"), 
         DateTuple::new(1958, 11, 22)),
         (OsStr::new("halloween.xml"),
         DateTuple::new(1988, 9, 22)),
         (OsStr::new("brasília_wallpapers.xml"),
         DateTuple::new(2002, 03, 22)), 
      ];
      let (mut f1, mut f2): (i32, i32);
      let mut resultados = vec![true];
      /* testes com vários períodos, que devem está
       * no arquivo de configuração, certo?! */
      for (feriado, data) in feriados.drain(..) {
         // data aleatória para teste de quase um ano.
         let mut t = 0i32;
         f1 = 0; f2 = 0;
         // testando as duas funções de uma só vez.
         for funcao in [parte_iv, parteIV] {
            let mut inicio = data.as_ref().unwrap().clone();
            for _ in 1..40 {
               // obtendo nova transição.
               let saida = funcao(inicio.clone());
               if saida.file_name().unwrap() == feriado {
                  if funcao == parte_iv
                     { f1 += 1; }
                  else
                     { f2 += 1; }
               } 
               t += 1;
               println!(
                  "data: {}\tseleção: {:#?}",
                  inicio.to_readable_string(),
                  saida.file_name().unwrap()
               );
               // avançando dia ...
               inicio = inicio.next_date();
            }
            print!("\n");
         }
         let p = (f1 as f32) / (t as f32);
         let P = (f2 as f32) / (t as f32);
         println!("{3:#?}\n{0:0.1}% ~ {1:0.1}%
            \rdiferença: {2}\n", 
            (P * 100.0), (p * 100.0),
            (f1 - f2).abs(),
            feriado
         );
         /* verifica se a dirença dos 'outputs'
          * estão pertos, por uma estreita margem
          * de erro. */
         resultados.push((f1-f2).abs() <= 5);
      }
      // restaurando do backup.
      salva_e_restaura(vec![HISTORICO, ULTIMA_MOD]);
      // espera-se uma diferença de inputs menor que cinco.
      assert!(resultados.iter().all(|vl| *vl));
   }
}
