#!/usr/bin/python3 -O

"""
  Pega todos os arquivos XML gerados até o momento registradas no banco de 
dados, então gera um novo, se ele ainda existir. Por quê? Bem, talvez 
tenham sido adicionados novas imagens em tal, então é melhor contabiliza-lás
e adicionar junto com as demais transições. Também a função geradora de 
tais configurações faz transições de maneira aleatório, se for refeito, 
gerará novas configurações com transições diferentes, isso é bom, pois tem 
uma variação de transições não sempre iguais, e com tempos também distintos.
"""

# bibliotecas externas:
from PIL import Image, UnidentifiedImageError
# biblioteca do programa.
from utilitarios import gera_configuracao_xml
# biblioteca padrão do Python:
from pathlib import PosixPath
import os.path, os, imghdr
from collections.abc import Sequence

# Raíz com todos subdiretórios contendo imagens e seus respectivos 
# arquivos XML.
CORE = os.getenv("HOME")
RAIZ = PosixPath(CORE, "Pictures")

def info_analise_do_diretorio(caminho: str, medias: Sequence[float, float], 
  percentual: float, aprovacao: bool) -> None:
   comprimento = len(caminho)
   MARGEM = 2
   (media_x, media_y) = medias
   dimensao_valida = (media_x >= 1_366 and media_y >= 768)

   print(
      """\n\rDiretório{0:.>{c}} 
      \rMédia de pixels{1:.>{ci}}x{2} 
      \rImagens válidas{3:.>{cii}}%
      \rFoi aprovado?{4:.>9s}
      """.format(
         caminho, int(media_x), int(media_y), 
         int(percentual*100), str(aprovacao),
         c = comprimento + MARGEM + 6,
         ci = 4 + MARGEM, cii = 3 + MARGEM
      )
   )
...

def busca_na_raiz() -> Sequence[PosixPath]:
   "Retorna lista com todos dirs válidos a gerar XML's."
   diretorios = []
   # todos diretórios neste diretório.
   arvore_do_diretorio = os.walk(RAIZ)

   for tupla in arvore_do_diretorio:
      # percentual de imagens válidas que foram iteradas e processadas.
      percentual = 0.0
      # média da largura/altura da imagem pixels.
      media_y = media_x = 0.0
      ha_subdiretorios = bool(tupla[1])

      # Pula o processamento de diretórios com subdiretórios.
      if ha_subdiretorios: continue

      for imagem in tupla[2]:
         diretorio = tupla[0]
         # Concatenando o caminho até tal diretório com o arquivo, para 
         # formar um caminho do arquivo.
         caminho_img = os.path.join(diretorio, imagem)
         qtd_de_arquivos = len(tupla[2])

         # dimensão de wallpaper:
         try:
            img = Image.open(caminho_img)
            # tipo de imagem, se for uma.
            identidade = img.format.lower() 
            imagem_e_valida = identidade in ("jpg", "jpeg", "png")

            if imagem_e_valida: 
               media_y += img.height / qtd_de_arquivos
               media_x += img.width / qtd_de_arquivos
               percentual += 1 / qtd_de_arquivos
            ...
         except UnidentifiedImageError: pass
      ...

      # Proposições para válida tal diretório:
      dimensao_da_imagem_e_valida = (media_x >= 1_366 and media_y >= 768)
      percentual_aceitavel = (0.90 <= percentual <= 1.0)

      if percentual_aceitavel and dimensao_da_imagem_e_valida:
         diretorios.append(tupla[0])

      # visualizando ánalise do processamento acima...
      info_analise_do_diretorio(tupla[0], (media_x, media_y), percentual,
         (percentual_aceitavel and dimensao_da_imagem_e_valida))
   ...
   return diretorios
...

def atualiza_na_raiz():
   "Faz atualização baseado na raíz com imagens cedida."
   for diretorio in busca_na_raiz():
      # Forma um caminho dado o nome e caminho registrado. Impressão do 
      # caminho, que deve ser "concatenada" com a impressão do resutado.
      if os.path.exists(diretorio):
         print("existe.")
         # Se existir, então vamos gerar um novo visando a atualização 
         # em sí.
         gera_configuracao_xml(diretorio)
      else:
         print("NÃO EXISTE!")
      ...
   ...
...


# execução do script de forma
def verificacao_da_listagem_de_diretorios_validos():
   for (num, xmls) in enumerate(busca_na_raiz()):
      #print(num + 1,'ª) ', xmls, sep = "")
      print("{:>3d}ª {}".format(num + 1, xmls))
   caminho = os.path.join(
      os.getenv("HOME"), "Pictures",
      "blocos_unicode_caractéres.png"
   )
...

if __debug__:
   verificacao_da_listagem_de_diretorios_validos()
else:
   # executar script propriamente.
   atualiza_na_raiz()
...
