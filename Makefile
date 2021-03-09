SHELL		:= /bin/sh
INSTALL		?= install

PREFIX		?= /usr/local
BINDIR		?= ${PREFIX}/bin
DATADIR		?= ${PREFIX}/share
MANDIR		?= ${DATADIR}/man/

TARGET		:= kmon

INSTALL_PROGRAM	:= $(INSTALL) -s
INSTALL_DATA	:= $(INSTALL) -m 0644

manpages	:= man/kmon.8

install: release ${manpages}
	$(INSTALL)         -d ${DESTDIR}/${BINDIR}
	$(INSTALL_PROGRAM) -t ${DESTDIR}/${BINDIR}      target/release/${TARGET}
	$(INSTALL)         -d ${DESTDIR}/${MANDIR}/man8
	$(INSTALL_DATA)    -t ${DESTDIR}/${MANDIR}/man8 ${manpages}
release:
	cargo build --release --locked


.PHONY: release install
