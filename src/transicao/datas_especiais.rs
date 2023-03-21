

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

type Cabecalho = HashMap<String, Vec<String>>;

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
            Some(mut atual) => 
               { atual.push(l.to_string()); } 
            None => 
               { println!("chave não existe, provavelmente espaço vázio."); }
         };
      }

      /* extrai conteúdo do cabeçalho, e cria
       * uma chave no dicionário com ela. Isso
       * aciona uma vez por cabeçalho. */
      if cabecalho_detectado {
         let i = l.find('[') + 1;
         let f = l.find(']');
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
