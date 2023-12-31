use std::collections::HashMap;
pub static LANG_NAMES: &[&str; 578] = &[
	"1C Enterprise",
	"4D",
	"ABAP",
	"ABAP CDS",
	"ABNF",
	"AGS Script",
	"AIDL",
	"AL",
	"AMPL",
	"ANTLR",
	"API Blueprint",
	"APL",
	"ASL",
	"ASN.1",
	"ASP.NET",
	"ATS",
	"ActionScript",
	"Ada",
	"Adobe Font Metrics",
	"Agda",
	"Alloy",
	"Altium Designer",
	"AngelScript",
	"ApacheConf",
	"Apex",
	"Apollo Guidance Computer",
	"AppleScript",
	"Arc",
	"AsciiDoc",
	"AspectJ",
	"Assembly",
	"Astro",
	"Asymptote",
	"Augeas",
	"AutoHotkey",
	"AutoIt",
	"Avro IDL",
	"Awk",
	"BASIC",
	"Ballerina",
	"Batchfile",
	"Beef",
	"Befunge",
	"BibTeX",
	"Bicep",
	"Bison",
	"BitBake",
	"Blade",
	"BlitzBasic",
	"BlitzMax",
	"Bluespec",
	"Boo",
	"Boogie",
	"Brainfuck",
	"Brightscript",
	"C",
	"C#",
	"C++",
	"C-ObjDump",
	"C2hs Haskell",
	"CIL",
	"CLIPS",
	"CMake",
	"COBOL",
	"COLLADA",
	"CSON",
	"CSS",
	"CSV",
	"CUE",
	"CWeb",
	"Cabal Config",
	"Cap'n Proto",
	"CartoCSS",
	"Ceylon",
	"Chapel",
	"Charity",
	"ChucK",
	"Cirru",
	"Clarion",
	"Classic ASP",
	"Clean",
	"Click",
	"Clojure",
	"Closure Templates",
	"CoNLL-U",
	"CodeQL",
	"CoffeeScript",
	"ColdFusion",
	"ColdFusion CFC",
	"Common Lisp",
	"Common Workflow Language",
	"Component Pascal",
	"Cool",
	"Coq",
	"Cpp-ObjDump",
	"Creole",
	"Crystal",
	"Csound",
	"Csound Document",
	"Csound Score",
	"Cuda",
	"Cue Sheet",
	"Cycript",
	"Cython",
	"D",
	"D-ObjDump",
	"DIGITAL Command Language",
	"DM",
	"DNS Zone",
	"DTrace",
	"Dafny",
	"Darcs Patch",
	"Dart",
	"DataWeave",
	"Dhall",
	"Diff",
	"DirectX 3D File",
	"Dockerfile",
	"Dogescript",
	"Dylan",
	"E",
	"E-mail",
	"EBNF",
	"ECL",
	"ECLiPSe",
	"EJS",
	"EQ",
	"Eagle",
	"Easybuild",
	"Ecere Projects",
	"Edje Data Collection",
	"Eiffel",
	"Elixir",
	"Elm",
	"Emacs Lisp",
	"EmberScript",
	"Erlang",
	"F#",
	"F*",
	"FIGlet Font",
	"FLUX",
	"Factor",
	"Fancy",
	"Fantom",
	"Faust",
	"Fennel",
	"Filebench WML",
	"Filterscript",
	"Fluent",
	"Formatted",
	"Forth",
	"Fortran",
	"Fortran Free Form",
	"FreeBasic",
	"FreeMarker",
	"Frege",
	"Futhark",
	"G-code",
	"GAML",
	"GAMS",
	"GAP",
	"GCC Machine Description",
	"GDB",
	"GDScript",
	"GEDCOM",
	"GLSL",
	"GN",
	"Game Maker Language",
	"Genie",
	"Genshi",
	"Gentoo Ebuild",
	"Gentoo Eclass",
	"Gerber Image",
	"Gettext Catalog",
	"Gherkin",
	"Git Config",
	"Glyph",
	"Glyph Bitmap Distribution Format",
	"Gnuplot",
	"Go",
	"Golo",
	"Gosu",
	"Grace",
	"Gradle",
	"Grammatical Framework",
	"Graph Modeling Language",
	"GraphQL",
	"Graphviz (DOT)",
	"Groovy",
	"Groovy Server Pages",
	"HAProxy",
	"HCL",
	"HLSL",
	"HTML",
	"HTML+ECR",
	"HTML+EEX",
	"HTML+ERB",
	"HTML+PHP",
	"HTML+Razor",
	"HTTP",
	"HXML",
	"Hack",
	"Haml",
	"Handlebars",
	"Harbour",
	"Haskell",
	"Haxe",
	"HiveQL",
	"HolyC",
	"Hy",
	"HyPhy",
	"IDL",
	"IGOR Pro",
	"INI",
	"IRC log",
	"Idris",
	"Ignore List",
	"ImageJ Macro",
	"Inform 7",
	"Inno Setup",
	"Io",
	"Ioke",
	"Isabelle",
	"J",
	"JFlex",
	"JSON",
	"JSON with Comments",
	"JSON5",
	"JSONLD",
	"JSONiq",
	"Jasmin",
	"Java",
	"Java Properties",
	"Java Server Pages",
	"JavaScript",
	"JavaScript+ERB",
	"Jest Snapshot",
	"Jinja",
	"Jison",
	"Jison Lex",
	"Jolie",
	"Jsonnet",
	"Julia",
	"Jupyter Notebook",
	"KRL",
	"Kaitai Struct",
	"KakouneScript",
	"KiCad Layout",
	"KiCad Legacy Layout",
	"KiCad Schematic",
	"Kit",
	"Kotlin",
	"Kusto",
	"LFE",
	"LLVM",
	"LOLCODE",
	"LSL",
	"LTspice Symbol",
	"LabVIEW",
	"Lark",
	"Lasso",
	"Latte",
	"Lean",
	"Less",
	"Lex",
	"LilyPond",
	"Limbo",
	"Linker Script",
	"Linux Kernel Module",
	"Liquid",
	"Literate Agda",
	"Literate CoffeeScript",
	"Literate Haskell",
	"LiveScript",
	"Logos",
	"Logtalk",
	"LookML",
	"LoomScript",
	"Lua",
	"M",
	"M4",
	"M4Sugar",
	"MATLAB",
	"MAXScript",
	"MLIR",
	"MQL4",
	"MQL5",
	"MTML",
	"MUF",
	"Macaulay2",
	"Makefile",
	"Mako",
	"Markdown",
	"Marko",
	"Mask",
	"Mathematica",
	"Max",
	"Mercury",
	"Metal",
	"Microsoft Developer Studio Project",
	"Microsoft Visual Studio Solution",
	"MiniD",
	"Mirah",
	"Modelica",
	"Modula-2",
	"Modula-3",
	"Module Management System",
	"Monkey",
	"Moocode",
	"MoonScript",
	"Motoko",
	"Motorola 68K Assembly",
	"Muse",
	"Mustache",
	"Myghty",
	"NASL",
	"NCL",
	"NEON",
	"NL",
	"NSIS",
	"NWScript",
	"Nearley",
	"Nemerle",
	"NetLinx",
	"NetLinx+ERB",
	"NetLogo",
	"NewLisp",
	"Nextflow",
	"Nginx",
	"Nim",
	"Ninja",
	"Nit",
	"Nix",
	"Nu",
	"NumPy",
	"Nunjucks",
	"OCaml",
	"ObjDump",
	"Object Data Instance Notation",
	"ObjectScript",
	"Objective-C++",
	"Objective-J",
	"Odin",
	"Omgrofl",
	"Opa",
	"Opal",
	"Open Policy Agent",
	"OpenCL",
	"OpenEdge ABL",
	"OpenQASM",
	"OpenSCAD",
	"OpenStep Property List",
	"OpenType Feature File",
	"Org",
	"Ox",
	"Oxygene",
	"Oz",
	"P4",
	"PEG.js",
	"PHP",
	"PLSQL",
	"PLpgSQL",
	"POV-Ray SDL",
	"Pan",
	"Papyrus",
	"Parrot",
	"Parrot Assembly",
	"Parrot Internal Representation",
	"Pascal",
	"Pawn",
	"Pep8",
	"Perl",
	"Pic",
	"Pickle",
	"PicoLisp",
	"PigLatin",
	"Pike",
	"PlantUML",
	"Pod",
	"Pod 6",
	"PogoScript",
	"Pony",
	"PostCSS",
	"PostScript",
	"PowerBuilder",
	"PowerShell",
	"Prisma",
	"Processing",
	"Proguard",
	"Prolog",
	"Promela",
	"Propeller Spin",
	"Protocol Buffer",
	"Public Key",
	"Pug",
	"Puppet",
	"Pure Data",
	"PureBasic",
	"PureScript",
	"Python",
	"Python traceback",
	"Q#",
	"QML",
	"QMake",
	"Qt Script",
	"R",
	"RAML",
	"RDoc",
	"REALbasic",
	"REXX",
	"RMarkdown",
	"RPC",
	"RPM Spec",
	"RUNOFF",
	"Racket",
	"Ragel",
	"Raku",
	"Rascal",
	"Raw token data",
	"ReScript",
	"Reason",
	"Rebol",
	"Red",
	"Redcode",
	"Regular Expression",
	"Ren'Py",
	"RenderScript",
	"Rich Text Format",
	"Ring",
	"Riot",
	"RobotFramework",
	"Roff",
	"Roff Manpage",
	"Rouge",
	"Ruby",
	"Rust",
	"SAS",
	"SCSS",
	"SELinux Policy",
	"SMT",
	"SPARQL",
	"SQF",
	"SQL",
	"SQLPL",
	"SRecode Template",
	"STON",
	"SVG",
	"SWIG",
	"Sage",
	"SaltStack",
	"Sass",
	"Scala",
	"Scaml",
	"Scheme",
	"Scilab",
	"Self",
	"ShaderLab",
	"Shell",
	"ShellSession",
	"Shen",
	"Sieve",
	"Slash",
	"Slice",
	"Slim",
	"SmPL",
	"Smali",
	"Smalltalk",
	"Smarty",
	"Solidity",
	"SourcePawn",
	"Spline Font Database",
	"Squirrel",
	"Stan",
	"Standard ML",
	"Starlark",
	"Stata",
	"StringTemplate",
	"Stylus",
	"SubRip Text",
	"SugarSS",
	"SuperCollider",
	"Svelte",
	"Swift",
	"SystemVerilog",
	"TI Program",
	"TLA",
	"TOML",
	"TSQL",
	"TSV",
	"TSX",
	"TXL",
	"Tcl",
	"Tcsh",
	"TeX",
	"Tea",
	"Terra",
	"Texinfo",
	"Text",
	"Textile",
	"Thrift",
	"Turing",
	"Turtle",
	"Twig",
	"Type Language",
	"TypeScript",
	"Unified Parallel C",
	"Unity3D Asset",
	"Unix Assembly",
	"Uno",
	"UnrealScript",
	"UrWeb",
	"V",
	"VBA",
	"VBScript",
	"VCL",
	"VHDL",
	"Vala",
	"Valve Data Format",
	"Verilog",
	"Vim Script",
	"Vim Snippet",
	"Visual Basic .NET",
	"Volt",
	"Vue",
	"Wavefront Material",
	"Wavefront Object",
	"Web Ontology Language",
	"WebAssembly",
	"WebIDL",
	"WebVTT",
	"Wikitext",
	"Windows Registry Entries",
	"Wollok",
	"World of Warcraft Addon Data",
	"X BitMap",
	"X PixMap",
	"X10",
	"XC",
	"XML",
	"XML Property List",
	"XPages",
	"XProc",
	"XQuery",
	"XS",
	"XSLT",
	"Xojo",
	"Xonsh",
	"Xtend",
	"YAML",
	"YANG",
	"YARA",
	"YASnippet",
	"Yacc",
	"ZAP",
	"ZIL",
	"Zeek",
	"ZenScript",
	"Zephir",
	"Zig",
	"Zimpl",
	"desktop",
	"dircolors",
	"eC",
	"edn",
	"fish",
	"jq",
	"mIRC Script",
	"mcfunction",
	"mupad",
	"nanorc",
	"nesC",
	"ooc",
	"q",
	"reStructuredText",
	"sed",
	"wdl",
	"wisp",
	"xBase",
];
pub fn get_language_index() -> HashMap<&'static str, usize> {
	HashMap::from([
		("bsl", 0),
		("os", 0),
		("4dm", 1),
		("abap", 2),
		("asddls", 3),
		("abnf", 4),
		("asc", 5),
		("ash", 5),
		("aidl", 6),
		("al", 7),
		("ampl", 8),
		("mod", 8),
		("g4", 9),
		("apib", 10),
		("apl", 11),
		("dyalog", 11),
		("asl", 12),
		("dsl", 12),
		("asn", 13),
		("asn1", 13),
		("asax", 14),
		("ascx", 14),
		("ashx", 14),
		("asmx", 14),
		("aspx", 14),
		("axd", 14),
		("dats", 15),
		("hats", 15),
		("sats", 15),
		("as", 16),
		("adb", 17),
		("ada", 17),
		("ads", 17),
		("afm", 18),
		("agda", 19),
		("als", 20),
		("OutJob", 21),
		("PcbDoc", 21),
		("PrjPCB", 21),
		("SchDoc", 21),
		("as", 22),
		("angelscript", 22),
		("apacheconf", 23),
		("vhost", 23),
		("cls", 24),
		("agc", 25),
		("applescript", 26),
		("scpt", 26),
		("arc", 27),
		("asciidoc", 28),
		("adoc", 28),
		("asc", 28),
		("aj", 29),
		("asm", 30),
		("a51", 30),
		("i", 30),
		("inc", 30),
		("nasm", 30),
		("astro", 31),
		("asy", 32),
		("aug", 33),
		("ahk", 34),
		("ahkl", 34),
		("au3", 35),
		("avdl", 36),
		("awk", 37),
		("auk", 37),
		("gawk", 37),
		("mawk", 37),
		("nawk", 37),
		("bas", 38),
		("bal", 39),
		("bat", 40),
		("cmd", 40),
		("bf", 41),
		("befunge", 42),
		("bib", 43),
		("bibtex", 43),
		("bicep", 44),
		("bison", 45),
		("bb", 46),
		("blade", 47),
		("blade.php", 47),
		("bb", 48),
		("decls", 48),
		("bmx", 49),
		("bsv", 50),
		("boo", 51),
		("bpl", 52),
		("b", 53),
		("bf", 53),
		("brs", 54),
		("c", 55),
		("cats", 55),
		("h", 55),
		("idc", 55),
		("cs", 56),
		("cake", 56),
		("csx", 56),
		("linq", 56),
		("cpp", 57),
		("c++", 57),
		("cc", 57),
		("cp", 57),
		("cxx", 57),
		("h++", 57),
		("hh", 57),
		("hpp", 57),
		("hxx", 57),
		("inc", 57),
		("inl", 57),
		("ino", 57),
		("ipp", 57),
		("ixx", 57),
		("re", 57),
		("tcc", 57),
		("tpp", 57),
		("c-objdump", 58),
		("chs", 59),
		("cil", 60),
		("clp", 61),
		("cmake", 62),
		("cmake.in", 62),
		("cob", 63),
		("cbl", 63),
		("ccp", 63),
		("cobol", 63),
		("cpy", 63),
		("dae", 64),
		("cson", 65),
		("css", 66),
		("csv", 67),
		("cue", 68),
		("w", 69),
		("cabal", 70),
		("capnp", 71),
		("mss", 72),
		("ceylon", 73),
		("chpl", 74),
		("ch", 75),
		("ck", 76),
		("cirru", 77),
		("clw", 78),
		("asp", 79),
		("icl", 80),
		("dcl", 80),
		("click", 81),
		("clj", 82),
		("boot", 82),
		("cl2", 82),
		("cljc", 82),
		("cljs", 82),
		("cljs.hl", 82),
		("cljscm", 82),
		("cljx", 82),
		("hic", 82),
		("soy", 83),
		("conllu", 84),
		("conll", 84),
		("ql", 85),
		("qll", 85),
		("coffee", 86),
		("_coffee", 86),
		("cake", 86),
		("cjsx", 86),
		("iced", 86),
		("cfm", 87),
		("cfml", 87),
		("cfc", 88),
		("lisp", 89),
		("asd", 89),
		("cl", 89),
		("l", 89),
		("lsp", 89),
		("ny", 89),
		("podsl", 89),
		("sexp", 89),
		("cwl", 90),
		("cp", 91),
		("cps", 91),
		("cl", 92),
		("coq", 93),
		("v", 93),
		("cppobjdump", 94),
		("c++-objdump", 94),
		("c++objdump", 94),
		("cpp-objdump", 94),
		("cxx-objdump", 94),
		("creole", 95),
		("cr", 96),
		("orc", 97),
		("udo", 97),
		("csd", 98),
		("sco", 99),
		("cu", 100),
		("cuh", 100),
		("cue", 101),
		("cy", 102),
		("pyx", 103),
		("pxd", 103),
		("pxi", 103),
		("d", 104),
		("di", 104),
		("d-objdump", 105),
		("com", 106),
		("dm", 107),
		("zone", 108),
		("arpa", 108),
		("d", 109),
		("dfy", 110),
		("darcspatch", 111),
		("dpatch", 111),
		("dart", 112),
		("dwl", 113),
		("dhall", 114),
		("diff", 115),
		("patch", 115),
		("x", 116),
		("dockerfile", 117),
		("djs", 118),
		("dylan", 119),
		("dyl", 119),
		("intr", 119),
		("lid", 119),
		("E", 120),
		("eml", 121),
		("mbox", 121),
		("ebnf", 122),
		("ecl", 123),
		("eclxml", 123),
		("ecl", 124),
		("ejs", 125),
		("ect", 125),
		("ejs.t", 125),
		("jst", 125),
		("eq", 126),
		("sch", 127),
		("brd", 127),
		("eb", 128),
		("epj", 129),
		("edc", 130),
		("e", 131),
		("ex", 132),
		("exs", 132),
		("elm", 133),
		("el", 134),
		("emacs", 134),
		("emacs.desktop", 134),
		("em", 135),
		("emberscript", 135),
		("erl", 136),
		("app.src", 136),
		("es", 136),
		("escript", 136),
		("hrl", 136),
		("xrl", 136),
		("yrl", 136),
		("fs", 137),
		("fsi", 137),
		("fsx", 137),
		("fst", 138),
		("flf", 139),
		("fx", 140),
		("flux", 140),
		("factor", 141),
		("fy", 142),
		("fancypack", 142),
		("fan", 143),
		("dsp", 144),
		("fnl", 145),
		("f", 146),
		("fs", 147),
		("ftl", 148),
		("for", 149),
		("eam.fs", 149),
		("fth", 150),
		("4th", 150),
		("f", 150),
		("for", 150),
		("forth", 150),
		("fr", 150),
		("frt", 150),
		("fs", 150),
		("f", 151),
		("f77", 151),
		("for", 151),
		("fpp", 151),
		("f90", 152),
		("f03", 152),
		("f08", 152),
		("f95", 152),
		("bi", 153),
		("bas", 153),
		("ftl", 154),
		("fr", 155),
		("fut", 156),
		("g", 157),
		("cnc", 157),
		("gco", 157),
		("gcode", 157),
		("gaml", 158),
		("gms", 159),
		("g", 160),
		("gap", 160),
		("gd", 160),
		("gi", 160),
		("tst", 160),
		("md", 161),
		("gdb", 162),
		("gdbinit", 162),
		("gd", 163),
		("ged", 164),
		("glsl", 165),
		("fp", 165),
		("frag", 165),
		("frg", 165),
		("fs", 165),
		("fsh", 165),
		("fshader", 165),
		("geo", 165),
		("geom", 165),
		("glslf", 165),
		("glslv", 165),
		("gs", 165),
		("gshader", 165),
		("rchit", 165),
		("rmiss", 165),
		("shader", 165),
		("tesc", 165),
		("tese", 165),
		("vert", 165),
		("vrx", 165),
		("vsh", 165),
		("vshader", 165),
		("gn", 166),
		("gni", 166),
		("gml", 167),
		("gs", 168),
		("kid", 169),
		("ebuild", 170),
		("eclass", 171),
		("gbr", 172),
		("cmp", 172),
		("gbl", 172),
		("gbo", 172),
		("gbp", 172),
		("gbs", 172),
		("gko", 172),
		("gml", 172),
		("gpb", 172),
		("gpt", 172),
		("gtl", 172),
		("gto", 172),
		("gtp", 172),
		("gts", 172),
		("ncl", 172),
		("sol", 172),
		("po", 173),
		("pot", 173),
		("feature", 174),
		("story", 174),
		("gitconfig", 175),
		("glf", 176),
		("bdf", 177),
		("gp", 178),
		("gnu", 178),
		("gnuplot", 178),
		("p", 178),
		("plot", 178),
		("plt", 178),
		("go", 179),
		("golo", 180),
		("gs", 181),
		("gst", 181),
		("gsx", 181),
		("vark", 181),
		("grace", 182),
		("gradle", 183),
		("gf", 184),
		("gml", 185),
		("graphql", 186),
		("gql", 186),
		("graphqls", 186),
		("dot", 187),
		("gv", 187),
		("groovy", 188),
		("grt", 188),
		("gtpl", 188),
		("gvy", 188),
		("gsp", 189),
		("cfg", 190),
		("hcl", 191),
		("nomad", 191),
		("tf", 191),
		("tfvars", 191),
		("workflow", 191),
		("hlsl", 192),
		("cginc", 192),
		("fx", 192),
		("fxh", 192),
		("hlsli", 192),
		("html", 193),
		("hta", 193),
		("htm", 193),
		("html.hl", 193),
		("inc", 193),
		("xht", 193),
		("xhtml", 193),
		("ecr", 194),
		("eex", 195),
		("html.heex", 195),
		("html.leex", 195),
		("erb", 196),
		("erb.deface", 196),
		("rhtml", 196),
		("phtml", 197),
		("cshtml", 198),
		("razor", 198),
		("http", 199),
		("hxml", 200),
		("hack", 201),
		("hh", 201),
		("hhi", 201),
		("php", 201),
		("haml", 202),
		("haml.deface", 202),
		("handlebars", 203),
		("hbs", 203),
		("hb", 204),
		("hs", 205),
		("hs-boot", 205),
		("hsc", 205),
		("hx", 206),
		("hxsl", 206),
		("q", 207),
		("hql", 207),
		("hc", 208),
		("hy", 209),
		("bf", 210),
		("pro", 211),
		("dlm", 211),
		("ipf", 212),
		("ini", 213),
		("cfg", 213),
		("dof", 213),
		("lektorproject", 213),
		("prefs", 213),
		("pro", 213),
		("properties", 213),
		("irclog", 214),
		("weechatlog", 214),
		("idr", 215),
		("lidr", 215),
		("gitignore", 216),
		("ijm", 217),
		("ni", 218),
		("i7x", 218),
		("iss", 219),
		("isl", 219),
		("io", 220),
		("ik", 221),
		("thy", 222),
		("ijs", 223),
		("flex", 224),
		("jflex", 224),
		("json", 225),
		("avsc", 225),
		("geojson", 225),
		("gltf", 225),
		("har", 225),
		("ice", 225),
		("JSON-tmLanguage", 225),
		("jsonl", 225),
		("mcmeta", 225),
		("tfstate", 225),
		("tfstate.backup", 225),
		("topojson", 225),
		("webapp", 225),
		("webmanifest", 225),
		("yy", 225),
		("yyp", 225),
		("jsonc", 226),
		("sublime-build", 226),
		("sublime-commands", 226),
		("sublime-completions", 226),
		("sublime-keymap", 226),
		("sublime-macro", 226),
		("sublime-menu", 226),
		("sublime-mousemap", 226),
		("sublime-project", 226),
		("sublime-settings", 226),
		("sublime-theme", 226),
		("sublime-workspace", 226),
		("sublime_metrics", 226),
		("sublime_session", 226),
		("json5", 227),
		("jsonld", 228),
		("jq", 229),
		("j", 230),
		("java", 231),
		("jav", 231),
		("properties", 232),
		("jsp", 233),
		("js", 234),
		("_js", 234),
		("bones", 234),
		("cjs", 234),
		("es", 234),
		("es6", 234),
		("frag", 234),
		("gs", 234),
		("jake", 234),
		("javascript", 234),
		("jsb", 234),
		("jscad", 234),
		("jsfl", 234),
		("jsm", 234),
		("jss", 234),
		("jsx", 234),
		("mjs", 234),
		("njs", 234),
		("pac", 234),
		("sjs", 234),
		("ssjs", 234),
		("xsjs", 234),
		("xsjslib", 234),
		("js.erb", 235),
		("snap", 236),
		("jinja", 237),
		("j2", 237),
		("jinja2", 237),
		("jison", 238),
		("jisonlex", 239),
		("ol", 240),
		("iol", 240),
		("jsonnet", 241),
		("libsonnet", 241),
		("jl", 242),
		("ipynb", 243),
		("krl", 244),
		("ksy", 245),
		("kak", 246),
		("kicad_pcb", 247),
		("kicad_mod", 247),
		("kicad_wks", 247),
		("brd", 248),
		("sch", 249),
		("kit", 250),
		("kt", 251),
		("ktm", 251),
		("kts", 251),
		("csl", 252),
		("lfe", 253),
		("ll", 254),
		("lol", 255),
		("lsl", 256),
		("lslp", 256),
		("asy", 257),
		("lvproj", 258),
		("lvlib", 258),
		("lark", 259),
		("lasso", 260),
		("las", 260),
		("lasso8", 260),
		("lasso9", 260),
		("latte", 261),
		("lean", 262),
		("hlean", 262),
		("less", 263),
		("l", 264),
		("lex", 264),
		("ly", 265),
		("ily", 265),
		("b", 266),
		("m", 266),
		("ld", 267),
		("lds", 267),
		("x", 267),
		("mod", 268),
		("liquid", 269),
		("lagda", 270),
		("litcoffee", 271),
		("coffee.md", 271),
		("lhs", 272),
		("ls", 273),
		("_ls", 273),
		("xm", 274),
		("x", 274),
		("xi", 274),
		("lgt", 275),
		("logtalk", 275),
		("lookml", 276),
		("model.lkml", 276),
		("view.lkml", 276),
		("ls", 277),
		("lua", 278),
		("fcgi", 278),
		("nse", 278),
		("p8", 278),
		("pd_lua", 278),
		("rbxs", 278),
		("rockspec", 278),
		("wlua", 278),
		("mumps", 279),
		("m", 279),
		("m4", 280),
		("m4", 281),
		("matlab", 282),
		("m", 282),
		("ms", 283),
		("mcr", 283),
		("mlir", 284),
		("mq4", 285),
		("mqh", 285),
		("mq5", 286),
		("mqh", 286),
		("mtml", 287),
		("muf", 288),
		("m", 288),
		("m2", 289),
		("mak", 290),
		("d", 290),
		("make", 290),
		("makefile", 290),
		("mk", 290),
		("mkfile", 290),
		("mako", 291),
		("mao", 291),
		("md", 292),
		("markdown", 292),
		("mdown", 292),
		("mdwn", 292),
		("mdx", 292),
		("mkd", 292),
		("mkdn", 292),
		("mkdown", 292),
		("ronn", 292),
		("scd", 292),
		("workbook", 292),
		("marko", 293),
		("mask", 294),
		("mathematica", 295),
		("cdf", 295),
		("m", 295),
		("ma", 295),
		("mt", 295),
		("nb", 295),
		("nbp", 295),
		("wl", 295),
		("wlt", 295),
		("maxpat", 296),
		("maxhelp", 296),
		("maxproj", 296),
		("mxt", 296),
		("pat", 296),
		("m", 297),
		("moo", 297),
		("metal", 298),
		("dsp", 299),
		("sln", 300),
		("minid", 301),
		("druby", 302),
		("duby", 302),
		("mirah", 302),
		("mo", 303),
		("mod", 304),
		("i3", 305),
		("ig", 305),
		("m3", 305),
		("mg", 305),
		("mms", 306),
		("mmk", 306),
		("monkey", 307),
		("monkey2", 307),
		("moo", 308),
		("moon", 309),
		("mo", 310),
		("x68", 311),
		("muse", 312),
		("mustache", 313),
		("myt", 314),
		("nasl", 315),
		("inc", 315),
		("ncl", 316),
		("neon", 317),
		("nl", 318),
		("nsi", 319),
		("nsh", 319),
		("nss", 320),
		("ne", 321),
		("nearley", 321),
		("n", 322),
		("axs", 323),
		("axi", 323),
		("axs.erb", 324),
		("axi.erb", 324),
		("nlogo", 325),
		("nl", 326),
		("lisp", 326),
		("lsp", 326),
		("nf", 327),
		("nginx", 328),
		("nginxconf", 328),
		("vhost", 328),
		("nim", 329),
		("nim.cfg", 329),
		("nimble", 329),
		("nimrod", 329),
		("nims", 329),
		("ninja", 330),
		("nit", 331),
		("nix", 332),
		("nu", 333),
		("numpy", 334),
		("numpyw", 334),
		("numsc", 334),
		("njk", 335),
		("ml", 336),
		("eliom", 336),
		("eliomi", 336),
		("ml4", 336),
		("mli", 336),
		("mll", 336),
		("mly", 336),
		("objdump", 337),
		("odin", 338),
		("cls", 339),
		("mm", 340),
		("j", 341),
		("sj", 341),
		("odin", 342),
		("omgrofl", 343),
		("opa", 344),
		("opal", 345),
		("rego", 346),
		("cl", 347),
		("opencl", 347),
		("p", 348),
		("cls", 348),
		("w", 348),
		("qasm", 349),
		("scad", 350),
		("plist", 351),
		("glyphs", 351),
		("fea", 352),
		("org", 353),
		("ox", 354),
		("oxh", 354),
		("oxo", 354),
		("oxygene", 355),
		("oz", 356),
		("p4", 357),
		("pegjs", 358),
		("php", 359),
		("aw", 359),
		("ctp", 359),
		("fcgi", 359),
		("inc", 359),
		("php3", 359),
		("php4", 359),
		("php5", 359),
		("phps", 359),
		("phpt", 359),
		("pls", 360),
		("bdy", 360),
		("ddl", 360),
		("fnc", 360),
		("pck", 360),
		("pkb", 360),
		("pks", 360),
		("plb", 360),
		("plsql", 360),
		("prc", 360),
		("spc", 360),
		("sql", 360),
		("tpb", 360),
		("tps", 360),
		("trg", 360),
		("vw", 360),
		("pgsql", 361),
		("sql", 361),
		("pov", 362),
		("inc", 362),
		("pan", 363),
		("psc", 364),
		("parrot", 365),
		("pasm", 366),
		("pir", 367),
		("pas", 368),
		("dfm", 368),
		("dpr", 368),
		("inc", 368),
		("lpr", 368),
		("pascal", 368),
		("pp", 368),
		("pwn", 369),
		("inc", 369),
		("sma", 369),
		("pep", 370),
		("pl", 371),
		("al", 371),
		("cgi", 371),
		("fcgi", 371),
		("perl", 371),
		("ph", 371),
		("plx", 371),
		("pm", 371),
		("psgi", 371),
		("t", 371),
		("pic", 372),
		("chem", 372),
		("pkl", 373),
		("l", 374),
		("pig", 375),
		("pike", 376),
		("pmod", 376),
		("puml", 377),
		("iuml", 377),
		("plantuml", 377),
		("pod", 378),
		("pod", 379),
		("pod6", 379),
		("pogo", 380),
		("pony", 381),
		("pcss", 382),
		("postcss", 382),
		("ps", 383),
		("eps", 383),
		("epsi", 383),
		("pfa", 383),
		("pbt", 384),
		("sra", 384),
		("sru", 384),
		("srw", 384),
		("ps1", 385),
		("psd1", 385),
		("psm1", 385),
		("prisma", 386),
		("pde", 387),
		("pro", 388),
		("pl", 389),
		("pro", 389),
		("prolog", 389),
		("yap", 389),
		("pml", 390),
		("spin", 391),
		("proto", 392),
		("asc", 393),
		("pub", 393),
		("jade", 394),
		("pug", 394),
		("pp", 395),
		("pd", 396),
		("pb", 397),
		("pbi", 397),
		("purs", 398),
		("py", 399),
		("cgi", 399),
		("fcgi", 399),
		("gyp", 399),
		("gypi", 399),
		("lmi", 399),
		("py3", 399),
		("pyde", 399),
		("pyi", 399),
		("pyp", 399),
		("pyt", 399),
		("pyw", 399),
		("rpy", 399),
		("smk", 399),
		("spec", 399),
		("tac", 399),
		("wsgi", 399),
		("xpy", 399),
		("pytb", 400),
		("qs", 401),
		("qml", 402),
		("qbs", 402),
		("pro", 403),
		("pri", 403),
		("qs", 404),
		("r", 405),
		("rd", 405),
		("rsx", 405),
		("raml", 406),
		("rdoc", 407),
		("rbbas", 408),
		("rbfrm", 408),
		("rbmnu", 408),
		("rbres", 408),
		("rbtbar", 408),
		("rbuistate", 408),
		("rexx", 409),
		("pprx", 409),
		("rex", 409),
		("rmd", 410),
		("x", 411),
		("spec", 412),
		("rnh", 413),
		("rno", 413),
		("rkt", 414),
		("rktd", 414),
		("rktl", 414),
		("scrbl", 414),
		("rl", 415),
		("6pl", 416),
		("6pm", 416),
		("nqp", 416),
		("p6", 416),
		("p6l", 416),
		("p6m", 416),
		("pl", 416),
		("pl6", 416),
		("pm", 416),
		("pm6", 416),
		("raku", 416),
		("rakumod", 416),
		("t", 416),
		("rsc", 417),
		("raw", 418),
		("res", 419),
		("re", 420),
		("rei", 420),
		("reb", 421),
		("r", 421),
		("r2", 421),
		("r3", 421),
		("rebol", 421),
		("red", 422),
		("reds", 422),
		("cw", 423),
		("regexp", 424),
		("regex", 424),
		("rpy", 425),
		("rsh", 426),
		("rtf", 427),
		("ring", 428),
		("riot", 429),
		("robot", 430),
		("roff", 431),
		("1", 431),
		("1in", 431),
		("1m", 431),
		("1x", 431),
		("2", 431),
		("3", 431),
		("3in", 431),
		("3m", 431),
		("3p", 431),
		("3pm", 431),
		("3qt", 431),
		("3x", 431),
		("4", 431),
		("5", 431),
		("6", 431),
		("7", 431),
		("8", 431),
		("9", 431),
		("l", 431),
		("man", 431),
		("mdoc", 431),
		("me", 431),
		("ms", 431),
		("n", 431),
		("nr", 431),
		("rno", 431),
		("tmac", 431),
		("1", 432),
		("1in", 432),
		("1m", 432),
		("1x", 432),
		("2", 432),
		("3", 432),
		("3in", 432),
		("3m", 432),
		("3p", 432),
		("3pm", 432),
		("3qt", 432),
		("3x", 432),
		("4", 432),
		("5", 432),
		("6", 432),
		("7", 432),
		("8", 432),
		("9", 432),
		("man", 432),
		("mdoc", 432),
		("rg", 433),
		("rb", 434),
		("builder", 434),
		("eye", 434),
		("fcgi", 434),
		("gemspec", 434),
		("god", 434),
		("jbuilder", 434),
		("mspec", 434),
		("pluginspec", 434),
		("podspec", 434),
		("prawn", 434),
		("rabl", 434),
		("rake", 434),
		("rbi", 434),
		("rbuild", 434),
		("rbw", 434),
		("rbx", 434),
		("ru", 434),
		("ruby", 434),
		("spec", 434),
		("thor", 434),
		("watchr", 434),
		("rs", 435),
		("rs.in", 435),
		("sas", 436),
		("scss", 437),
		("te", 438),
		("smt2", 439),
		("smt", 439),
		("sparql", 440),
		("rq", 440),
		("sqf", 441),
		("hqf", 441),
		("sql", 442),
		("cql", 442),
		("ddl", 442),
		("inc", 442),
		("mysql", 442),
		("prc", 442),
		("tab", 442),
		("udf", 442),
		("viw", 442),
		("sql", 443),
		("db2", 443),
		("srt", 444),
		("ston", 445),
		("svg", 446),
		("i", 447),
		("sage", 448),
		("sagews", 448),
		("sls", 449),
		("sass", 450),
		("scala", 451),
		("kojo", 451),
		("sbt", 451),
		("sc", 451),
		("scaml", 452),
		("scm", 453),
		("sch", 453),
		("sld", 453),
		("sls", 453),
		("sps", 453),
		("ss", 453),
		("sci", 454),
		("sce", 454),
		("tst", 454),
		("self", 455),
		("shader", 456),
		("sh", 457),
		("bash", 457),
		("bats", 457),
		("cgi", 457),
		("command", 457),
		("env", 457),
		("fcgi", 457),
		("ksh", 457),
		("sh.in", 457),
		("tmux", 457),
		("tool", 457),
		("zsh", 457),
		("sh-session", 458),
		("shen", 459),
		("sieve", 460),
		("sl", 461),
		("ice", 462),
		("slim", 463),
		("cocci", 464),
		("smali", 465),
		("st", 466),
		("cs", 466),
		("tpl", 467),
		("sol", 468),
		("sp", 469),
		("inc", 469),
		("sfd", 470),
		("nut", 471),
		("stan", 472),
		("ml", 473),
		("fun", 473),
		("sig", 473),
		("sml", 473),
		("bzl", 474),
		("do", 475),
		("ado", 475),
		("doh", 475),
		("ihlp", 475),
		("mata", 475),
		("matah", 475),
		("sthlp", 475),
		("st", 476),
		("styl", 477),
		("srt", 478),
		("sss", 479),
		("sc", 480),
		("scd", 480),
		("svelte", 481),
		("swift", 482),
		("sv", 483),
		("svh", 483),
		("vh", 483),
		("8xp", 484),
		("8xk", 484),
		("8xk.txt", 484),
		("8xp.txt", 484),
		("tla", 485),
		("toml", 486),
		("sql", 487),
		("tsv", 488),
		("tsx", 489),
		("txl", 490),
		("tcl", 491),
		("adp", 491),
		("tcl.in", 491),
		("tm", 491),
		("tcsh", 492),
		("csh", 492),
		("tex", 493),
		("aux", 493),
		("bbx", 493),
		("cbx", 493),
		("cls", 493),
		("dtx", 493),
		("ins", 493),
		("lbx", 493),
		("ltx", 493),
		("mkii", 493),
		("mkiv", 493),
		("mkvi", 493),
		("sty", 493),
		("toc", 493),
		("tea", 494),
		("t", 495),
		("texinfo", 496),
		("texi", 496),
		("txi", 496),
		("txt", 497),
		("fr", 497),
		("nb", 497),
		("ncl", 497),
		("no", 497),
		("textile", 498),
		("thrift", 499),
		("t", 500),
		("tu", 500),
		("ttl", 501),
		("twig", 502),
		("tl", 503),
		("ts", 504),
		("upc", 505),
		("anim", 506),
		("asset", 506),
		("mask", 506),
		("mat", 506),
		("meta", 506),
		("prefab", 506),
		("unity", 506),
		("s", 507),
		("ms", 507),
		("uno", 508),
		("uc", 509),
		("ur", 510),
		("urs", 510),
		("v", 511),
		("bas", 512),
		("cls", 512),
		("frm", 512),
		("frx", 512),
		("vba", 512),
		("vbs", 513),
		("vcl", 514),
		("vhdl", 515),
		("vhd", 515),
		("vhf", 515),
		("vhi", 515),
		("vho", 515),
		("vhs", 515),
		("vht", 515),
		("vhw", 515),
		("vala", 516),
		("vapi", 516),
		("vdf", 517),
		("v", 518),
		("veo", 518),
		("vim", 519),
		("vba", 519),
		("vimrc", 519),
		("vmb", 519),
		("snip", 520),
		("snippet", 520),
		("snippets", 520),
		("vb", 521),
		("vbhtml", 521),
		("volt", 522),
		("vue", 523),
		("mtl", 524),
		("obj", 525),
		("owl", 526),
		("wast", 527),
		("wat", 527),
		("webidl", 528),
		("vtt", 529),
		("mediawiki", 530),
		("wiki", 530),
		("wikitext", 530),
		("reg", 531),
		("wlk", 532),
		("toc", 533),
		("xbm", 534),
		("xpm", 535),
		("pm", 535),
		("x10", 536),
		("xc", 537),
		("xml", 538),
		("plist", 539),
		("stTheme", 539),
		("tmCommand", 539),
		("tmLanguage", 539),
		("tmPreferences", 539),
		("tmSnippet", 539),
		("tmTheme", 539),
		("xsp-config", 540),
		("xsp.metadata", 540),
		("xpl", 541),
		("xproc", 541),
		("xquery", 542),
		("xq", 542),
		("xql", 542),
		("xqm", 542),
		("xqy", 542),
		("xs", 543),
		("xslt", 544),
		("xsl", 544),
		("xojo_code", 545),
		("xojo_menu", 545),
		("xojo_report", 545),
		("xojo_script", 545),
		("xojo_toolbar", 545),
		("xojo_window", 545),
		("xsh", 546),
		("xtend", 547),
		("yml", 548),
		("mir", 548),
		("reek", 548),
		("rviz", 548),
		("sublime-syntax", 548),
		("syntax", 548),
		("yaml", 548),
		("yaml-tmlanguage", 548),
		("yaml.sed", 548),
		("yml.mysql", 548),
		("yang", 549),
		("yar", 550),
		("yara", 550),
		("yasnippet", 551),
		("y", 552),
		("yacc", 552),
		("yy", 552),
		("zap", 553),
		("xzap", 553),
		("zil", 554),
		("mud", 554),
		("zeek", 555),
		("bro", 555),
		("zs", 556),
		("zep", 557),
		("zig", 558),
		("zimpl", 559),
		("zmpl", 559),
		("zpl", 559),
		("desktop", 560),
		("desktop.in", 560),
		("dircolors", 561),
		("ec", 562),
		("eh", 562),
		("edn", 563),
		("fish", 564),
		("jq", 565),
		("mrc", 566),
		("mcfunction", 567),
		("mu", 568),
		("nanorc", 569),
		("nc", 570),
		("ooc", 571),
		("q", 572),
		("rst", 573),
		("rest", 573),
		("rest.txt", 573),
		("rst.txt", 573),
		("sed", 574),
		("wdl", 575),
		("wisp", 576),
		("prg", 577),
		("ch", 577),
		("prw", 577),
	])
}
