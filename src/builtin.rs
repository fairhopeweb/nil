macro_rules! def {
    (__impl $name:tt $kind:tt) => {
        Builtin {
            name: $name,
            kind: BuiltinKind::$kind,
        }
    };
    ($($name:literal $(= $kind:tt)?,)*) => {
        phf::phf_map! {
            $($name => def!(__impl $name $($kind)?),)*
        }
    };
}

pub static BUILTINS: phf::Map<&'static str, Builtin> = def! {
    "__addErrorContext" = Function,
    "__all" = Function,
    "__any" = Function,
    "__appendContext" = Function,
    "__attrNames" = Function,
    "__attrValues" = Function,
    "__bitAnd" = Function,
    "__bitOr" = Function,
    "__bitXor" = Function,
    "__catAttrs" = Function,
    "__ceil" = Function,
    "__compareVersions" = Function,
    "__concatLists" = Function,
    "__concatMap" = Function,
    "__concatStringsSep" = Function,
    "__currentSystem" = Const,
    "__currentTime" = Const,
    "__deepSeq" = Function,
    "__div" = Function,
    "__elem" = Function,
    "__elemAt" = Function,
    "__fetchurl" = Function,
    "__filter" = Function,
    "__filterSource" = Function,
    "__findFile" = Function,
    "__floor" = Function,
    "__foldl'" = Function,
    "__fromJSON" = Function,
    "__functionArgs" = Function,
    "__genList" = Function,
    "__genericClosure" = Function,
    "__getAttr" = Function,
    "__getContext" = Function,
    "__getEnv" = Function,
    "__getFlake" = Function,
    "__groupBy" = Function,
    "__hasAttr" = Function,
    "__hasContext" = Function,
    "__hashFile" = Function,
    "__hashString" = Function,
    "__head" = Function,
    "__intersectAttrs" = Function,
    "__isAttrs" = Function,
    "__isBool" = Function,
    "__isFloat" = Function,
    "__isFunction" = Function,
    "__isInt" = Function,
    "__isList" = Function,
    "__isPath" = Function,
    "__isString" = Function,
    "__langVersion" = Const,
    "__length" = Function,
    "__lessThan" = Function,
    "__listToAttrs" = Function,
    "__mapAttrs" = Function,
    "__match" = Function,
    "__mul" = Function,
    "__nixPath" = Const,
    "__nixVersion" = Const,
    "__parseDrvName" = Function,
    "__partition" = Function,
    "__path" = Function,
    "__pathExists" = Function,
    "__readDir" = Function,
    "__readFile" = Function,
    "__replaceStrings" = Function,
    "__seq" = Function,
    "__sort" = Function,
    "__split" = Function,
    "__splitVersion" = Function,
    "__storeDir" = Const,
    "__storePath" = Function,
    "__stringLength" = Function,
    "__sub" = Function,
    "__substring" = Function,
    "__tail" = Function,
    "__toFile" = Function,
    "__toJSON" = Function,
    "__toPath" = Function,
    "__toXML" = Function,
    "__trace" = Function,
    "__traceVerbose" = Function,
    "__tryEval" = Function,
    "__typeOf" = Function,
    "__unsafeDiscardOutputDependency" = Function,
    "__unsafeDiscardStringContext" = Function,
    "__unsafeGetAttrPos" = Function,
    "__zipAttrsWith" = Function,

    "abort" = Function,
    "baseNameOf" = Function,
    "break" = Function,
    "builtins" = Attrset,
    "derivation" = Function,
    "derivationStrict" = Function,
    "dirOf" = Function,
    "false" = Const,
    "fetchGit" = Function,
    "fetchMercurial" = Function,
    "fetchTarball" = Function,
    "fetchTree" = Function,
    "fromTOML" = Function,
    "import" = Function,
    "isNull" = Function,
    "map" = Function,
    "null" = Const,
    "placeholder" = Function,
    "removeAttrs" = Function,
    "scopedImport" = Function,
    "throw" = Function,
    "toString" = Function,
    "true" = Const,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Builtin {
    pub name: &'static str,
    pub kind: BuiltinKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuiltinKind {
    Const,
    Function,
    Attrset,
}
