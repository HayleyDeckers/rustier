
# just run at crate root, because R is silly
# Rscript ./examples/libtest.R

path_relative <- "./target/debug/examples/"
filename <- "libtest"

OS <- Sys.info()["sysname"]

if (OS == "Windows") {
    extension <- ".dll"
} else if (OS == "Mac OS") { # technically also other options for Mac OS
    extension <- ".dylib"
} else {
    extension <- ".so"
}

lib_path <- paste0(path_relative, filename, extension) # cat, but not broken

dyn.load(lib_path) # load rust-generated dynamic library

# very slightly less clunkily calling those generated functions
external <- function(name, ...) {
    .External(sprintf("_RINTEROP_%s", name), ...)
}

external("hello_world")
