

/*
 Como a datas especiais, que por serem
 especiais, ganham exceção na seleções
 no dia que são marcadas, como dias que
 os antecedem.

 Para marcar alguma é preciso a data de 
 início, como o dia final(que aqui é a
 data especial em sí). O nome do arquivo
 é o 'namespace', se ele não existe na 
 busca, não será selecionado. O nome da
 'variável' têm que ser o nome do arquivo
 XML, e tem que está antes do sinal de 
 igual, já o 'intervalo', deve está à
 frente do nome da variável(que é o mesmo
 nome do arquivo XML, sem a extensão),
 separado por dois pontos. A sintaxe dele
 é uma primeira data que vai até outra(
 sendo está a data especial em sí) ambas 
 na forma de calendário, mas sem o ano 
 explicitado; sendo ambas separadas pela 
 preposição 'à' onde cada declaração desta
 estará em sua devida linha. Obviamente 
 elas não podém ser trocadas de ordem, se 
 forem o programa vai detectar, e apitar erro.


 Dias das 'datas especiais' não podem
 se sobrepor, mais os intervalos cruzados
 o podem.

 Tudo isso têm que está no arquivo, localizado
 na raíz rotulado como 'data_especiais.conf';
 abaixo do cabeçalho "Datas Especiais". O 
 arquivo futuramente, tanto mudará de nome, para
 algo mais genérico como "configuração.conf", pois,
 como se pode ver no cabeçalho, servirá como
 configuração geral do programa, então não será
 colocado apenas o tal cabeçalho citado lá.
 */

// biblioteca padrão:
use std::collections::HashMap;
use std::str::FromStr;
// biblioteca externa:
use date_time::date_tuple::DateTuple;

// melhora codificação:
type Cabecalho = HashMap<String, Vec<String>>;
type LinhaData = (String, DateTuple, DateTuple);


/* separa um cabeçalho, e as linhas ligadas à ele,
 * por meio de um dicionário. */
fn todas_configuracoes(conteudo: String) -> Cabecalho {
   let mut cabecalho_detectado = false;
   /* dicionário que com 'chave de cabeçalho',
    * coleta linhas baixo dele. */
   let mut mapa = Cabecalho::new();
   let mut chave: String = "".to_string();

   for l in conteudo.lines() {
      // se char um novo cabeçalho.
      if l.contains(&"[") && l.contains(&"]") 
         { cabecalho_detectado = true; } 
      // acessa atual chave.
      else { 
         match mapa.get_mut(&chave) {
            Some(mut atual) => { 
               let linha = {
                  l.trim_matches('\r')
                  .trim_matches('\t')
                  .trim()
                  .to_string()
               };
               if linha != ""
                  { atual.push(linha); }
            } 
            None => 
               { println!("chave não existe, provavelmente espaço vázio."); }
         };
      }

      /* extrai conteúdo do cabeçalho, e cria
       * uma chave no dicionário com ela. Isso
       * aciona uma vez por cabeçalho. */
      if cabecalho_detectado {
         let i = l.find('[').unwrap() + 1;
         let f = l.find(']').unwrap();
         let cabecalho = l.get(i..f).unwrap().to_string();
         // mudando nova chave.
         chave = cabecalho.clone();
         mapa.insert(cabecalho, vec![]);
         // desativa até um próximo cabeçalho.
         cabecalho_detectado = false;
      }
   }

   return mapa;
}

/* pega slice-string formatando uma data(dia e mês)
 * e a transforma nesta especificamente baseado na 
 * estrutura adequada, desconsidera o ano, e coloca
 * sempre como o atual. */
fn extrai_data(string: &str) -> DateTuple {
   let partes = string.split_once("/").unwrap();
   let mes = u8::from_str(partes.1.trim()).unwrap();
   let dia = u8::from_str(partes.0.trim()).unwrap();
   let atual = DateTuple::today();
   DateTuple::new(atual.get_year(), mes, dia).unwrap()
}
/* parse linha contendo nome do arquivo, e
 * período de atual. */
fn parse_linha(linha: String) -> LinhaData {
   let partes = linha.split_once(':').unwrap();

   let mut arquivo_xml = {
      partes.0.trim()
      .to_string()
   };
   arquivo_xml.push_str(".xml");

   // pegando a data e, extraindo em duas.
   let subpartes = partes.1.split_once("à").unwrap();
   let data_inicio = extrai_data(subpartes.0);
   let data_fim = extrai_data(subpartes.1);

   return (arquivo_xml, data_inicio, data_fim);
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn saida_sastifatoria() {
      let mut entradas = vec![
         // uma bem simples.
         "\r[Datas Especias]
         \r   corpos_cristes: 05/04 à 23/05
         \r   dia_de_sao_miguel: 11/01 à 12/01
         ".to_string(),
         // bem mais espaçada que a anterior.
         "[DATAS ESPECIAIS]

         corpos_cristes: 05/04 à 23/05
         dia_de_sao_miguel: 11/01 à 12/01



         ".to_string(),
         // vários cabeçalhos e espaçadas.
         "[Configuração]
            hora: 12AM

            data: 13/05
            cor: azul


         [Datas Especias]
            corpos_cristes: 05/04 à 23/05
            dia_de_sao_miguel: 11/01 à 12/01


            dia_do_yourgut: 26/01
         [Diretórios Apropriados]
         pasta1 : '/home/etc'

         pasta2 : '/home/Videos/casas'


         pasta3 : '/root/secreto/muito_secreto'
         ".to_string()
      ];

      for entrada in entradas.drain(..) {
         let resultado = todas_configuracoes(entrada);
         println!("{:#?}", resultado);
      }
   }

   #[test]
   fn transformacao_linhas_datas() {
      let mut entradas = vec![
         "saint_patrick_day: 03/03 à 15/03".to_string(),
         "boogyman: 15/06 à 04/07".to_string(),
         "maria_rosa: 28/10 à 31/10".to_string(),
         // entradas meios desorganizadas nos espaços:
         "dia-de-santa-luzia  : 20/06  à      10/07".to_string(),
         "domingo_sagrado :  12/09       à 18/09".to_string(),
         "   milagra-são-nunca: 11/05  à  23/05".to_string()
      ];

      let atual = DateTuple::today();
      let ano = atual.get_year();

      assert_eq!(
         parse_linha(entradas.pop().unwrap()), 
         ("milagra-são-nunca.xml".to_string(), 
           DateTuple::new(ano, 05, 11).unwrap(),
           DateTuple::new(ano, 5, 23).unwrap()
         )
      );
      assert_eq!(
         parse_linha(entradas.pop().unwrap()), 
         ("domingo_sagrado.xml".to_string(), 
           DateTuple::new(ano, 9, 12).unwrap(),
           DateTuple::new(ano, 9, 18).unwrap()
         )
      );
      assert_eq!(
         parse_linha(entradas.pop().unwrap()), 
         ("dia-de-santa-luzia.xml".to_string(), 
           DateTuple::new(ano, 06, 20).unwrap(),
           DateTuple::new(ano, 7, 10).unwrap()
         )
      );
      assert_eq!(
         parse_linha(entradas.pop().unwrap()), 
         ("maria_rosa.xml".to_string(), 
           DateTuple::new(ano, 10, 28).unwrap(),
           DateTuple::new(ano, 10, 31).unwrap()
         )
      );
      assert_eq!(
         parse_linha(entradas.pop().unwrap()), 
         ("boogyman.xml".to_string(), 
           DateTuple::new(ano, 06, 15).unwrap(),
           DateTuple::new(ano, 07, 4).unwrap()
         )
      );
      assert_eq!(
         parse_linha(entradas.pop().unwrap()), 
         ("saint_patrick_day.xml".to_string(), 
           DateTuple::new(ano, 3, 3).unwrap(),
           DateTuple::new(ano, 3, 15).unwrap()
         )
      );
   }
}
