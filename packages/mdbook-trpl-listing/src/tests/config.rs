//! Check that the config options are correctly handled.
//!
//! Note: none of these tests particularly exercise the *wiring*. They just
//! assume that the config itself is done correctly. This is a small enough
//! chunk of code that it easy to verify by hand at present. If it becomes
//! more complex in the future, it would be good to revisit and integrate
//! the same kinds of tests as the unit tests above here.

use super::*;

// TODO: what *should* the behavior here be? I *think* it should error,
// in that there is a problem if it is invoked without that info.
#[test]
fn no_config() {
    let input_json = r##"[
                {
                    "root": "/path/to/book",
                    "config": {
                        "book": {
                            "authors": ["AUTHOR"],
                            "language": "en",
                            "multilingual": false,
                            "src": "src",
                            "title": "TITLE"
                        },
                        "preprocessor": {}
                    },
                    "renderer": "html",
                    "mdbook_version": "0.4.21"
                },
                {
                    "sections": [
                        {
                            "Chapter": {
                                "name": "Chapter 1",
                                "content": "# Chapter 1\n",
                                "number": [1],
                                "sub_items": [],
                                "path": "chapter_1.md",
                                "source_path": "chapter_1.md",
                                "parent_names": []
                            }
                        }
                    ],
                    "__non_exhaustive": null
                }
            ]"##;
    let input_json = input_json.as_bytes();
    let (ctx, book) =
        mdbook::preprocess::CmdPreprocessor::parse_input(input_json).unwrap();
    let result = TrplListing.run(&ctx, book);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(format!("{err}"), "No config for trpl-listing");
}

#[test]
fn empty_config() {
    let input_json = r##"[
                {
                    "root": "/path/to/book",
                    "config": {
                        "book": {
                            "authors": ["AUTHOR"],
                            "language": "en",
                            "multilingual": false,
                            "src": "src",
                            "title": "TITLE"
                        },
                        "preprocessor": {
                            "trpl-listing": {}
                        }
                    },
                    "renderer": "html",
                    "mdbook_version": "0.4.21"
                },
                {
                    "sections": [
                        {
                            "Chapter": {
                                "name": "Chapter 1",
                                "content": "# Chapter 1\n",
                                "number": [1],
                                "sub_items": [],
                                "path": "chapter_1.md",
                                "source_path": "chapter_1.md",
                                "parent_names": []
                            }
                        }
                    ],
                    "__non_exhaustive": null
                }
            ]"##;
    let input_json = input_json.as_bytes();
    let (ctx, book) =
        mdbook::preprocess::CmdPreprocessor::parse_input(input_json).unwrap();
    let result = TrplListing.run(&ctx, book);
    assert!(result.is_ok());
}

#[test]
fn specify_default() {
    let input_json = r##"[
                {
                    "root": "/path/to/book",
                    "config": {
                        "book": {
                            "authors": ["AUTHOR"],
                            "language": "en",
                            "multilingual": false,
                            "src": "src",
                            "title": "TITLE"
                        },
                        "preprocessor": {
                            "trpl-listing": {
                                "output-mode": "default"
                            }
                        }
                    },
                    "renderer": "html",
                    "mdbook_version": "0.4.21"
                },
                {
                    "sections": [
                        {
                            "Chapter": {
                                "name": "Chapter 1",
                                "content": "# Chapter 1\n",
                                "number": [1],
                                "sub_items": [],
                                "path": "chapter_1.md",
                                "source_path": "chapter_1.md",
                                "parent_names": []
                            }
                        }
                    ],
                    "__non_exhaustive": null
                }
            ]"##;
    let input_json = input_json.as_bytes();
    let (ctx, book) =
        mdbook::preprocess::CmdPreprocessor::parse_input(input_json).unwrap();
    let result = TrplListing.run(&ctx, book);
    assert!(result.is_ok());
}

#[test]
fn specify_simple() {
    let input_json = r##"[
                {
                    "root": "/path/to/book",
                    "config": {
                        "book": {
                            "authors": ["AUTHOR"],
                            "language": "en",
                            "multilingual": false,
                            "src": "src",
                            "title": "TITLE"
                        },
                        "preprocessor": {
                            "trpl-listing": {
                                "output-mode": "simple"
                            }
                        }
                    },
                    "renderer": "html",
                    "mdbook_version": "0.4.21"
                },
                {
                    "sections": [
                        {
                            "Chapter": {
                                "name": "Chapter 1",
                                "content": "# Chapter 1\n",
                                "number": [1],
                                "sub_items": [],
                                "path": "chapter_1.md",
                                "source_path": "chapter_1.md",
                                "parent_names": []
                            }
                        }
                    ],
                    "__non_exhaustive": null
                }
            ]"##;
    let input_json = input_json.as_bytes();
    let (ctx, book) =
        mdbook::preprocess::CmdPreprocessor::parse_input(input_json).unwrap();
    let result = TrplListing.run(&ctx, book);
    assert!(result.is_ok());
}

#[test]
fn specify_invalid() {
    let input_json = r##"[
                {
                    "root": "/path/to/book",
                    "config": {
                        "book": {
                            "authors": ["AUTHOR"],
                            "language": "en",
                            "multilingual": false,
                            "src": "src",
                            "title": "TITLE"
                        },
                        "preprocessor": {
                            "trpl-listing": {
                                "output-mode": "nonsense"
                            }
                        }
                    },
                    "renderer": "html",
                    "mdbook_version": "0.4.21"
                },
                {
                    "sections": [
                        {
                            "Chapter": {
                                "name": "Chapter 1",
                                "content": "# Chapter 1\n",
                                "number": [1],
                                "sub_items": [],
                                "path": "chapter_1.md",
                                "source_path": "chapter_1.md",
                                "parent_names": []
                            }
                        }
                    ],
                    "__non_exhaustive": null
                }
            ]"##;
    let input_json = input_json.as_bytes();
    let (ctx, book) =
        mdbook::preprocess::CmdPreprocessor::parse_input(input_json).unwrap();
    let result = TrplListing.run(&ctx, book);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(
        format!("{err}"),
        "Bad config value '\"nonsense\"' for key 'output-mode'"
    );
}
