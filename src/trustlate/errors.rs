#[derive(Debug)]
pub enum TrustlateError {
    OpenConfigFile,
    ParseConfigFile,
    InitCreateConfigFile,
    InitWriteConfigFile,
    InitCreateSourceDir,
    InitCreateTargetDir,
    InitCreateTranslationsFile,
    InitWriteTranslationsExample,
    ParseTranslationFileCannotOpen,
    ParseTraslationFileInvalidJson,
    ParseTranslationFileRepeatedLanguageKey,
    FixTreeCannotOpenSourceFile,
    FixTreeCannotWriteToSourceFile,
    GenerateCannotCreateOutputFile,
    GenerateCannotCreateOutputFolders,
    GenerateCannotWriteToOutputFile,
    GenerateCannotGenerateCode,
}
