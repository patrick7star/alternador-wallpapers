/*!
 * imprime e cria uma string que cuida
 * de, dado uma tupla com valores imprimiveis
 * faz tal impressão na forma de escada.
 */

// biblioteca do Rust:
use std::fmt::{Formatter, Display, Result as Resultado};
use std::dbg;
// biblioteca externa:
extern crate terminal_size;
use terminal_size::{terminal_size, Height, Width};


pub fn escadinha(entradas:Vec<&str>) -> String {
    /*
    let mut nova_str:String = String::from(array[0]);
    let mut acumulado:u16 = array[0].len() as u16;
    nova_str.push('\n'); // colocando quebra de linha.
    let mut repeticao = " ".repeat(acumulado as usize);
    nova_str.push_str(&repeticao[..]);
    nova_str.push_str(array[1]);
    nova_str.push_str("\n");
    acumulado += (array[1].len()) as u16;

    repeticao = " ".repeat(acumulado as usize);
    nova_str.push_str(&repeticao[..]);
    nova_str.push_str(array[2]);
    nova_str.push_str("\n");
    acumulado += (array[2].len()) as u16;

    repeticao = " ".repeat(acumulado as usize);
    nova_str.push_str(&repeticao[..]);
    nova_str.push_str(array[3]);
    nova_str.push_str("\n");
    acumulado += (array[3].len()) as u16;

    repeticao = " ".repeat(acumulado as usize);
    nova_str.push_str(&repeticao[..]);
    nova_str.push_str(array[4]);
    nova_str.push_str("\n");
    */
   // largura do terminal.
   let largura_tela = match terminal_size() {
      Some((Width(l), Height(_))) => l,
      None => 0u16
   };

   let mut nova_str:String = String::new();
   let mut acumulado:u16 = 0;
   let mut colidiu_na_parede:bool = false;
   let mut pilha_de_recuos:Vec<u16> = Vec::with_capacity(100);

   for s in entradas.iter() {
      let repeticao = " ".repeat(acumulado as usize);
      // adiciona "vácuo".
      nova_str.push_str(&repeticao[..]);
      // adiciona string.
      nova_str.push_str(s);
      // quebra de linha.
      nova_str.push('\n');
      //let meio_str:u16;
      if (s.len() as u16 + acumulado < largura_tela) &&
      !colidiu_na_parede {
         // cotabilizando espaço recuado.
         let meio_str:u16 = (s.len() / 2) as u16;
         acumulado += meio_str;
         /* adiciona aculuma, para quando bater na tela, 
          * o movimento de retrocesso comece. E tal, 
          * dará-se por tirar acumulu adicionado, ou seja
          * o primeiro a entrar será o último a sair,
          * uma pilha! */
         pilha_de_recuos.push(meio_str);
      }
      else {
         /* tira o acumulo colocado no topo da pilha,
          * e subtrai, criando como consequência um
          * movimento inverso. */
         if let Some(q) = pilha_de_recuos.pop() {
            acumulado -= q;
            // marca com colídiu com a parede direita, como certo.
            colidiu_na_parede = true;
         } else 
            { colidiu_na_parede = false }
      }
   }
    return nova_str;
}

/** imprime entradas de forma a preencher toda 
 a janela, onda a saida de texto está saindo. */
pub struct Impressao<'a> {
   // fila que contém um limite.
   fila:Vec<&'a str>,
   // quantidade e colunas a preencher.
   colunas:u8,
   // média de tamanho das strings inseridas.
   largura_media:u8,
   // maior comprimento de string medido.
   maior_comprimento:u8,
}

/// acrescimo para distânciar entradas.
const X:u8 = 3;

impl <'a>Impressao<'a> {
   // método construtor.
   pub fn cria(inicial:Vec<&'a str>) -> Self {
      // computando uma média inicial.
      let media:u8 = { 
         // somando tudo...
         let soma:usize = inicial.iter()
         .map(|s| s.len()).sum();
         // contabilizado o total de objetos.
         let total = inicial.len();
         // efetuando cálculo, e já convertendo...
         (soma / total) as u8
      };
      // computando o comprimento antigo.
      let comprimento:u8 = {
         inicial.iter()
         .map(|s| s.len())
         .max()
         .unwrap() as u8
      };
      // largura do terminal.
      let largura:u8 = match terminal_size() {
         Some((Width(l), Height(_))) => dbg!(l) as u8,
         None => 0_u8
      };
      #[allow(unused_variables)]
      // instânciando objeto...
      Impressao {
         fila: inicial,
         largura_media: dbg!(media),
         colunas: dbg!(largura/(comprimento + X)),
         maior_comprimento: dbg!(comprimento)
      }
   }
   
   // adiciona uma nova entrada para visualização.
   pub fn adiciona_entrada(&mut self, entrada:&'a str) {
      println!("nova entrada adicionada.");
      self.fila.push(entrada);
      // medindo comprimento e computando atual média.
      self.largura_media = { 
         let soma:usize = self.fila.iter()
         .map(|s| s.len())
         .sum();
         let total = self.fila.len();
         (soma / total) as u8
      };
      // só alterar maior entrada se... for maior.
      if self.maior_comprimento < entrada.len() as u8 {
         self.maior_comprimento = entrada.len() as u8;
      }
   }
}

impl Display for Impressao<'_> {
   fn fmt(&self, formatador:&mut Formatter<'_>) -> Resultado {
      // string para concatenação.
      let mut string = String::new();

      // iterando com fila, o primeiro inserido irá primeiro.
      let mut iterador = self.fila.iter();
      let mut n:u8 = 1;
      // enquanto o iterador não for inteiramente consumido...
      while let Some(s) = iterador.next() {
         // concatenando string...
         string.push_str(s);
         // insere quebra de linha no final da quarta coluna.
         if n % self.colunas == 0 
            { string.push('\n'); }
         else { 
            // computando o espaço adequado.
            let espaco:usize = {
               // comprimento da atual string, também apelido.
               let cs:u8 = s.len() as u8;
               // alias para "maior comprimento".
               let mcs:u8 = self.maior_comprimento;
               /* se for menor, ascrescenta a diferença e
                * o acrescimo constante 'X' básico. */
               if cs < mcs 
                  { ((mcs-cs) + X) as usize }
               // caso contrário só o acréscimo 'X'.
               else { X as usize }
            };
            // adiciona espaçamento.
            string.push_str(&" ".repeat(espaco)); 
         }
         n += 1;
      }
      // colocando quebra de linha final, se necessário.
      string += "\n";

      // escrevendo no output o resultado.
      write!(formatador, "{}", string)
   }
}


#[cfg(test)]
mod tests {
   // importando módulo acima.
   use crate::escadinha::*;

   #[test]
   fn teste_basico_escadinha() {
       let strings = vec!["era", "uma","casa", "muito","engraçada"];
       println!("visualizando:\n{}", escadinha(strings));
       assert!(true);
   }

   #[test]
   fn verificando_reverso() {
      let strings:Vec<&str> = vec![
         "era", "uma","casa", "muito","engraçada",
         "vaco", "original","casarao", "muito", "computador",
         "fio", "ovo","buraco", "armario", "caixas-de-som",
         "barata", "faca","xicara", "teclado", "quadrado",
         "sem-fio", "ovario", "sofa", "armario", "porte",
         "heranca", "magica","cabrito", "muito","engracada",
         "barata", "canivete","viacao", "tubulucao", "retangulo",
         "barata", "faca","xicara", "teclado", "quadrado",
         "sem-fio", "ovario", "sofa", "armario", "porte",
         "heranca", "magica","cabrito", "muito","engracada",
         "barata", "canivete","viacao", "tubulucao", "retangulo",
         "barata", "faca","xicara", "teclado", "quadrado",
         "sem-fio", "ovario", "sofa", "armario", "porte",
         "heranca", "magica","cabrito", "muito","engracada",
         "barata", "canivete","viacao", "tubulucao", "retangulo",
      ];
      println!("visualizando:\n{}", escadinha(strings));
      assert!(true);
   }

   #[test]
   fn teste_impressao_struct_parte_i() {
      let dirs:Vec<&str> = vec![
         "pasta_1", "pasta_2", "pasta_3", "pasta_4",
         "pasta_5", "pasta_6", "pasta_7", "pasta_8",
         "pasta_9", "pasta_10", "pasta_11", "pasta_12",
      ];

      let mut visual = Impressao::cria(dirs);
      println!("{}", visual);

      println!("verificando com fica a adição de mais entradas");
      visual.adiciona_entrada("pasta 14");
      visual.adiciona_entrada("pasta 15");
      visual.adiciona_entrada("pasta 16");
      println!("{}", visual);

      assert!(true);
   }

   #[test]
   fn outside_mudando_impressao() {
      let dirs:Vec<&str> = vec![
         "pasta_1", "pasta_322", "pasta_3", "pasta_4",
         "pasta_5", "pasta_6", "pasta", "pasta_8",
         "pasta_9", "pasta_12120", "pasta_11", "pasta_12",
      ];

      let visual = Impressao::cria(dirs);

      println!("{}", visual);
      assert!(true);
   }
}
