//go:build CGO_OS && CGO_ARCH

package PKG_NAME

// #cgo LDFLAGS: -L${SRCDIR}/lib/CGO_OS_CGO_ARCH -lPKG_NAME CGO_LDFLAGS_EXTRA
import "C"
