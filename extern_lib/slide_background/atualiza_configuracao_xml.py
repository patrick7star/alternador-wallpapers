#!/usr/bin/python3 -O

"""
pega todos os arquivos XML gerados até o
momento registradas no banco de dados, então
gera um novo, se ele ainda existir. Por quê?
Bem, talvez tenham sido adicionados novas
imagens em tal, então é melhor contabiliza-lás
e adicionar junto com as demais transições.
Também a função geradora de tais configurações
faz transições de maneira aleatório, se for 
refeito, gerará novas configurações com transições
diferentes, isso é bom, pois tem uma variação
de transições não sempre iguais, e com tempos
também distintos.
"""

# bibliotecas externas:
from PIL import Image, UnidentifiedImageError
# biblioteca do programa.
from utilitarios import gera_configuracao_xml

# biblioteca padrão do Python:
import os.path, os, imghdr

# raíz com todos subdiretórios contendo 
# imagens e seus respectivos arquivos XML.
RAIZ = os.path.join(os.getenv("HOME"), "Pictures")

def busca_na_raiz():
   # lista com todos dirs válidos a gerar
   # XML's.
   diretorios = []
   # todos diretórios neste diretório.
   arvore_do_diretorio = os.walk(RAIZ)

   for tupla in arvore_do_diretorio:
      # percentual de imagens válidas que 
      # foram iteradas e processadas.
      percentual = 0.0
      # média da largura da imagem pixels.
      media_x = 0.0
      # média da altura da imagem em pixels.
      media_y = 0.0
      if not bool(tupla[1]):
         for imagem in tupla[2]:
            # concatenando o caminho até tal diretório
            # com o arquivo, para formar um caminho
            # do arquivo.
            caminho_img = os.path.join(tupla[0], imagem)
            # total de arquivos no diretório.
            qtd = len(tupla[2])
            # dimensão de wallpaper:
            try:
               img = Image.open(caminho_img)
               # tipo de imagem, se for uma.
               identidade = img.format.lower() 
               # se o arquivo imagem é válido.
               imagem_e_valida = identidade in ("jpg", "jpeg", "png")
               if imagem_e_valida: 
                  media_y += img.height/qtd
                  media_x += img.width/qtd
                  percentual += 1/qtd
               ...
            except UnidentifiedImageError:
               pass
            ...
         ...
      ...
      # se ficar no percentual, então é um
      # diretório válido.
      if (0.90 <= percentual <= 1 and 
      media_x >= 1_366 and 
      media_y >= 768):
         diretorios.append(tupla[0])

      # visualizando...
      comprimento = len(tupla[0])
      MARGEM = 2
      print(
         """\n\rdiretório:{0:>{c}} 
         \rmédia de pixels:{1:>{ci}}x{2} 
         \rimagens válidas:{3:>{cii}}%\n"""
         .format(
            tupla[0], 
            int(media_x), 
            int(media_y), 
            int(percentual*100),
            c = comprimento + MARGEM + 6,
            ci = 4 + MARGEM,
            cii = 3 + MARGEM
         )
      )
   ...
            
   # retorna todos os xmls encontrados 
   # no diretório RAIZ, ou seus subdir's.
   return diretorios
...

# faz atualização baseado na raíz cedida.
def atualiza_na_raiz():
   for arquivo_xml in busca_na_raiz():
      # forma um caminho dado o nome e caminho registrado.
      # impressão do caminho, que deve ser "concatenada"
      # com a impressão do resutado.
      if os.path.exists(arquivo_xml):
         print("existe.")
         # se existir, então vamos gerar um novo
         # visando a atualização em sí.
         gera_configuracao_xml(arquivo_xml)
      else:
         print("NÃO EXISTE!")
      ...
   ...
...


# execução do script de forma
if __debug__:
   for (num, xmls) in enumerate(busca_na_raiz()):
      print(num+1,'ª) ', xmls, sep = "")
   caminho = os.path.join(
      os.getenv("HOME"), "Pictures",
      "blocos_unicode_caractéres.png"
   )
else:
   # executar script propriamente.
   atualiza_na_raiz()
...
