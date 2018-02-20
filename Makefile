#
# Copyright 2016, NICTA
#
# This software may be distributed and modified according to the terms of
# the BSD 2-Clause license. Note that NO WARRANTY is provided.
# See "LICENSE_BSD2.txt" for details.
#
# @TAG(NICTA_BSD)
#

include tools/camkes/toplevel.mk

.PHONY: qemu-arm

#qemu task, requires a kzm config to be used.
qemu-arm: capdl-loader-experimental-image
	qemu-system-arm -M kzm -nographic -kernel images/capdl-loader-experimental-image-arm-imx31

qemu-i386: capdl-loader-experimental-image
	qemu-system-i386 -cpu Haswell -m 512 -nographic -kernel images/kernel-ia32-pc99 -initrd images/capdl-loader-experimental-image-ia32-pc99
