# Generated by extendr: Do not edit by hand

# nolint start

#
# This file was created with the following call:
#   .Call("wrap__make_orbweaverRextras_wrappers", use_symbols = TRUE, package_name = "orbweaverRextras")

#' @usage NULL
#' @useDynLib orbweaverRextras, .registration = TRUE
NULL

#' Given a DAG return a dataframe with the leaves and parents
#' of a node vector
#' @export
get_leaves_as_df <- function(dag, nodes) .Call(wrap__get_leaves_as_df, dag, nodes)


# nolint end
