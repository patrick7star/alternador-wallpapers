
/*! 
código que dá utilitário que retorna barra
de progresso, dados uma quantia atual e total
a ser atingida. 
*/


// módulo externo:
//mod legivel;
//use self::legivel::tamanho;
use super::legivel::tamanho;
use std::time::{Duration, Instant};
use std::fmt::{Display, Formatter, Result as Result_fmt};
use std::ops::Range;


// símbolo que representa frações das barras e espaços vázios.
const COMPONENTE:&'static str ="#";
const VAZIO:&'static str=".";
const CAPACIDADE:u8 = 50;

/* cria uma proporção de progresso baseado 
 * na porcentagem dada. Então 0% é nada de
 * barra, e, 100% é a barra totalmente preenchida. */
fn cria_barra(percentagem:f32) -> String {
   let mut barra = String::new();
   let conta = (CAPACIDADE as f32 * percentagem) as usize;
   // falta de espaços-vázios.
   let diferenca:usize = 50 - conta;
   // concantena partes da barra.
   barra.push_str(&COMPONENTE.repeat(conta));
   barra.push_str(&VAZIO.repeat(diferenca));
   // retorna a barra formada com sua parte "consumida"
   // e uma parte "vázia", ou algum destes predominante.
   return barra;
}


fn conta_algs(valor:usize) -> u8 {
   let mut d:f32 = valor as f32;  // cópia valor real ...
   let mut contador:u8 = 0; // contador de divisões
   while d > 1.0 {
      // cada divisão por dez, conta um.
      d = d / 10.0;
      contador += 1;
   }
   // retorno a contagem contabilizando um ...
   return contador+1;
}


/// cálculos percentuais e, suas representação
/// numérica.
pub fn progresso(qtd_atual:u64, qtd_total:u64) -> String {
   let percentagem:f32 = (qtd_atual as f32)/(qtd_total as f32);
   let percent_100:f32 = percentagem*100.0;

   // caso de erro.
   if percentagem > 1.0_f32 {
      panic!("os valores de atual supera o total!");
   }
   else if percentagem == 1_f32 {
       return format!(
          "{0} de {1} [{2}]{3}\n", 
          qtd_total, qtd_total,
          cria_barra(1.0),100.0
      );
   }

   // molde da string retornada representando por 
   // inteiro a barra de progresso.
   // qtd. de algarismos que será alcançado o valor atual.
   let qtd_algs:usize = (conta_algs(qtd_total as usize)) as usize;
   let molde:String = format!(
      "{0:>espaco$} de {1} [{2}]{3:>5.1}%",
      qtd_atual, qtd_total, 
      cria_barra(percentagem), 
      percent_100,espaco=qtd_algs
   );
   return molde;
}


/// cálculos percentuais e, suas representação
/// numérica.
pub fn progresso_data(qtd_atual:u64, qtd_total:u64) -> String {
   let percentagem:f32 = (qtd_atual as f32)/(qtd_total as f32);
   let percent_100:f32 = percentagem*100.0;

   // caso de erro.
   if percentagem > 1.0_f32 {
      panic!("os valores de atual supera o total!");
   }
   else if percentagem == 1.00 {
      let total_bytes = tamanho(qtd_total, true);
      return format!(
         "{}/{} [{}] {}%\n",
         total_bytes,
         total_bytes,
         cria_barra(1.0),
         100.0
      );
   }
   else {
      // strings dos valores.
      let qa = tamanho(qtd_atual, true);
      let qt = tamanho(qtd_total, true);
      let qtd_algs = qt.len() as usize;
      let molde:String = format!(
         "{0:>espaco$}/{1} [{2}]{3:>5.1}%",
         qa, qt, cria_barra(percentagem), 
         percent_100,espaco=qtd_algs
      );
      return molde;
   }
}

pub struct Logo<'a> {
   // para marcar o tempo.
   ti:Instant,
   // o texto que será mostrado.
   rotulo:&'a str,
   // quando da string mostrar.
   capacidade:u8,
   // inicio e fim onde visualizar a string.
   ponta_esquerda:u8,
   ponta_direita:u8,
   // intervalo válido.
   intervalo:Option<Range<usize>>,
}
impl <'a> Logo<'a> {
   // criando uma nova instância.
   pub fn novo(label:&str) -> Result<Logo, &'static str> {
      if label.len() == 0 {
         Err("não é permitido strings em branco")
      }
      else {
         Ok(
            Logo {
               // iniciando contagem.
               ti: Instant::now(),
               // pegando o rótulo a dimanizar.
               rotulo: label,
               // capacidade definida manualmente.
               capacidade: 15, // quinze caractéres.
               ponta_esquerda: 0,
               ponta_direita: 15,
               intervalo:Some(0..15),
            }
         )
      }
   }
   // motor do logo. 
   pub fn movimenta_letreiro(&mut self) {
      // se chegou ao final, resetar posição do LED.
      if self.ponta_direita == self.rotulo.len() as u8 {
         self.ponta_direita = self.capacidade;
         self.ponta_esquerda = 0;
      }
      // a cada 1,5seg mover o led 'uma casa'.
      if self.ti.elapsed() > Duration::from_millis(500) {
         if self.ponta_direita <= self.rotulo.len() as u8 {
            // deslocando led...
            self.ponta_esquerda += 1;
            self.ponta_direita += 1;
            // resetando contagem...
            self.ti = Instant::now();
         }
      }
      // definindo novo intervalo.
      self.intervalo = {
         // "renomeação" para melhor legibilidade.
         let pe:usize = self.ponta_esquerda as usize;
         let pd:usize = self.ponta_direita as usize;
         Some(pe..pd)
      };
   }
   // transforma numa slice-string.
   pub fn para_string(&self) -> &'a str {
      match self.intervalo.clone() {
         Some(i) => {
            self.rotulo
            .get(i)
            .unwrap()
         },
         None => self.rotulo,
      }
   }
}
impl Display for Logo<'_> {
   fn fmt(&self, f:&mut Formatter<'_>) -> Result_fmt {
      // apeliando para legibilidade.
      match self.intervalo.clone() {
         Some(i) => {
            write!(
               f, "{}...",
               self.rotulo.get(i)
               .unwrap()
            )
         },
         None => write!(f, "{}", self.rotulo)
      }
   }
}

/** progresso com rótulo dinâmico, então quando
  o rótulo é muito maior do que cabe na tela, ele
  move o rótulo tipo aqueles slogans de neon em
  mercearias e shops. Ele também é de dados, o 
  resto do seu núcleo.
*/
pub fn progresso_data_rotulo<'a>(rotulo:&'a str, qtd_atual:u64, 
qtd_total:u64) -> String {
   // cálculando a porcetagem.
   let percentagem:f32 = (qtd_atual as f32)/(qtd_total as f32);
   // caso de erro.
   if percentagem > 1.0_f32 {
      panic!("os valores de atual supera o total!");
   }
   else if percentagem == 1f32 {
      let total_bytes = tamanho(qtd_total, true);
      return format!(
         "[{}] {}/{} [{}] {}%\n",
         rotulo,
         total_bytes,
         total_bytes,
         cria_barra(1.0),
         100.0
      );
   }
   else {
      // strings dos valores.
      let qa = tamanho(qtd_atual, true);
      let qt = tamanho(qtd_total, true);
      let qtd_algs = qt.len() as usize;
      let molde:String = format!(
         "[{4}] {0:>espaco$}/{1} [{2}]{3:>5.1}%",
         qa, qt, cria_barra(percentagem), 
         percentagem*100.0,
         rotulo,
         espaco=qtd_algs,
      );
      return molde;
   }
}


#[cfg(test)]
mod tests {
   #[test]
   fn teste_progresso_com_rotulo() {
      let rotulo:&str = "isso e um teste basico, sem panico";
      let mut logo:super::Logo = super::Logo::novo(rotulo).unwrap();
      for k in 1..(600_000+1) { 
         let bp = super::progresso_data_rotulo(
            logo.para_string(), 
            k, 600_000
         );
         print!("\r{}",bp); 
         logo.movimenta_letreiro();
      }
      assert!(true);
   }

   #[test]
   fn letreiro_dinamico() {
      // instanciando logo dinâmico...
      let texto = "isso e apenas um texto de teste, entao nao entre em panico";
      let mut logo = super::Logo::novo(texto).unwrap();
      // marcador de tempo.
      let t:super::Instant = super::Instant::now();
      while t.elapsed() < super::Duration::from_secs(15) {
         print!("\r{}", logo.para_string());
         logo.movimenta_letreiro();
      }
      assert!(true);
   }

   use std::thread::sleep;
   #[test]
   fn testando_funcao_que_gira() {
      let texto = "eu adoro suco de caju";
      let mut logo = super::Logo::novo(texto).unwrap();
      // movimento o texto, dormindo de acordo com
      // o tempo de translação dele(simulando tempo).
      sleep(super::Duration::from_secs_f32(0.5));
      logo.movimenta_letreiro();
      sleep(super::Duration::from_secs_f32(0.5));
      logo.movimenta_letreiro();
      sleep(super::Duration::from_secs_f32(0.5));
      logo.movimenta_letreiro();
      // tirando trecho translado.
      let parte_i = logo.para_string();
      /* o previsto, levando o tempo, e, 
       * lembrando que o "LED" tem 14 locais,
       * tem que cair exatamente como a 
       * frase abaixo.
       */
      let parte_ii = "adoro suco de c";
      // verificando resposta.
      assert_eq!(parte_i, parte_ii);
   }
}
