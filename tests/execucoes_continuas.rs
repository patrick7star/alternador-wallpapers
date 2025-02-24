extern crate utilitarios;

use utilitarios::aleatorio::{sortear};
use std::time::{Duration, Instant};


#[test]
fn nova_alternancia_continua() {
   /* copiando código, e alternando alguns 
    * valores para algo mais próximo de se 
    * verificar e, ocultando chamdas importantes
    * por simples mensagens para nenhuma mudança
    * relevante ser realmente feita. */
   const MINIMO:u16 = 5;
   const MAXIMO:u16 = 8 ;
   let mut cronometro = Instant::now();
   let tf:u16 = sortear::u16(MINIMO..=MAXIMO);
   let tempo_final = Duration::from_secs(tf as u64);
   let mut qtd = 0;

   'execucao:loop {
      if cronometro.elapsed() > tempo_final {
         //alterna_transicao();
         print!("aplicando nova transição ... ");
         print!("chama:'alterna_transicao'");
         println!(" ... pronto!");
         println!(
            "tempo decorrido:{0:3.2}seg\n", 
            cronometro
            .elapsed()
            .as_secs_f32()
         );
         cronometro = Instant::now();
         qtd += 1;
      }
      // quebra o laço após algumas visualizações.
      if qtd > 10 { break 'execucao; }
   }
   assert!(true);
}
