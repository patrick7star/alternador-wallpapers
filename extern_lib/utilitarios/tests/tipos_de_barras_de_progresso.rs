
/* teste conjunto de todos tipos de 
 * progressos criados.
 */
extern crate utilitarios;

/* o primeiro tipo criado, e consequente,
 * o mais simples. */
#[test]
//#[ignore]
fn progresso_simples() {
   println!("a \"barra de progresso\" mais básica:\n\t'intervalo de quase um milhão'");
   // fim do progresso.
   let total:u64 = 792_382;
   // laço finito.
   for k in 1..(total+1) {
      let bp:String =  {
         utilitarios
         ::barra_de_progresso
         ::progresso(k, total)
      };
      print!("\r{}", bp);
   }
   // sem quebra de linha, pois a função que
   // gera a string a mostrar, já cuida disto.
   assert!(true);
}

/* barra de progresso que mostra baseado em
 * dados a dinâmica da computabilidade. Ou 
 * seja, está aqui é um pouco mais complexo
 * em detalhar o progresso. */
#[test]
fn progresso_em_dados() {
   println!("a \"barra de progresso de dados\":");
   for k in 1..360_582 {
      let str_bp_data:String = {
         utilitarios
         ::barra_de_progresso
         ::progresso_data(k as u64, 360_582+1)
      };
      print!("\r{}", str_bp_data);
   }
   assert!(true)
}

/* este tipo de barra aceito um logo dinâmico
 * em que o texto fica em movimento se ele não
 * couber inteiramente na tela; isto da direita
 * para esquerda */
#[test]
fn progresso_em_dados_tipo_download() {
   println!("a \"barra de progresso com rótulo\":");
   let mut texto:utilitarios::barra_de_progresso::Logo = {
      utilitarios::
      barra_de_progresso
      ::Logo
      ::novo("GTA IX - New Gangs on the Street, Las Vegas") 
      .unwrap()
   };
   for k in 1..1_360_582+1 {
      // apelido para legibilidade.
      let s:String = texto.to_string();
      // extraindo texto do logo-dinâmico.
      let str_bp_data:String = {
         utilitarios
         ::barra_de_progresso
         ::progresso_data_rotulo(s.as_str(), k as u64, 1_360_582)
      };
      /* movendo logo, no tempo pre-determinado 
       * pelo programação. Acho que é meio segundo,
       * porém não importa, aqui só chama para que
       * se compute a translação do texto. */
      texto.movimenta_letreiro();
      print!("\r{}", str_bp_data);
   }
   assert!(true)
}
