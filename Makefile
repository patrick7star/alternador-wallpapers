VERSAO 	= 1.0.1
CAMINHO  = ../versões/personalizacao

salva:
	tar --exclude=target --exclude=Cargo.lock --exclude=LICENSE \
   --exclude=README.md -cvf $(CAMINHO).v$(VERSAO).tar *
	@echo "Backup da versão $(VERSAO) realizado com sucesso."

backups:
	@echo "\nListagem de todos backups feitos no computador:\n"
	@ls --human-readable --size --sort=time -1 $(CAMINHO)*.tar

debug:
	cargo build --verbose

release-cargo:
	cargo build --verbose --release

# Como todas bibliotecas já estão compiladas, tal processo é bem mais rápido.
# Ele compila o objeto do'main' somente, e linca nos compilados.
release-rustc:
	@rustc --edition 2021 --crate-name alterandor_wallpapers src/main.rs \
	-C opt-level=3 -C opt-level=z	-C strip=symbols								\
	-Llib/linux_x86_64																	\
		--extern 'utilitarios=lib/linux_x86_64/libutilitarios.rlib'			\
		--extern 'date_time=lib/linux_x86_64/libdate_time.rlib'				\
		--extern 'lazy_static=lib/linux_x86_64/liblazy_static.rlib'			\
		--extern 'regex=lib/linux_x86_64/libregex.rlib'							\
		--extern 'memchr=lib/linux_x86_64/libmemchr.rlib'						\
		--extern 'regex_automata=lib/linux_x86_64/libregex_automata.rlib'	\
		--extern 'aho_corasick=lib/linux_x86_64/libiaho_corasick.rlib'		\
		--extern 'regex_syntax=lib/linux_x86_64/libregex_syntax.rlib'		\
		--extern 'itoa=lib/linux_x86_64/libitoa.rlib'							\
		--extern 'serde_json=lib/linux_x86_64/libserde_json.rlib'			\
		--extern 'ryu=lib/linux_x86_64/libryu.rlib'								\
		--extern 'serde=lib/linux_x86_64/libserde.rlib'							\
		--extern 'libc=lib/linux_x86_64/liblibc.rlib'							\
	-o target/release/alternador_wallpapers
	@echo "Compila manualmente usando bibliotecas estáticas já compilada."

script-tests:
	@python3 -m unittest scripts.converte_historico_de_escolhas_para_json.Unitarios.test_lista_das_antigas_escolhas
	@python3 -m unittest scripts.converte_historico_de_escolhas_para_json.Unitarios.caminhos_bem_estabelecidos
