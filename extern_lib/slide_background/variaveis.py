
# dados importantes:
# nome do arquivo que contém perfils das criações anteriores.
registro_geral = 'registro_ssc.dat' 
# formatações de strings:
parte_estatica = """
   <static>
      <duration>{}</duration>
      <file>{}</file>
   </static>"""
parte_transicao = """
   <transition>
      <duration>{}</duration>
      <from>{}</from>
      <to>{}</to>
   </transition>\n""" 
tempo_inicializacao = """
   <starttime>
      <year>{}</year>
      <month>{}</month>
      <day>{}</day>
      <hour>{}</hour>
      <minute>{}</minute>
      <second>{}</second>
   </starttime>\n"""
texto_info = """
 nome: "{0}"
 caminho das imagens: "{1}"
 qtd. de imagens: {2}
 tempo de apresentação de uma imagem: {3} seg
 tempo de transição entres imagens: {4} seg
 horário de criação: {h}h {m}min {s}seg 
 data de criação: {dia}/{mes}/{ano}
 visualizando transição:\n{5}\n"""
