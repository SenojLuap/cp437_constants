#![allow(dead_code)]

// 00-0F
const NUL: char                                                 = '\u{0000}';
const WHITE_SMILING_FACE: char                                  = '☺';
const BLACK_SMILING_FACE: char                                  = '☻';
const BLACK_HEART_SUIT: char                                    = '♥';
const BLACK_DIAMOND_SUIT: char                                  = '♦';
const BLACK_CLUB_SUIT: char                                     = '♣';
const BLACK_SPADE_SUIT: char                                    = '♠';
const BULLET: char                                              = '•';
const INVERSE_BULLET: char                                      = '◘';
const WHITE_CIRCLE: char                                        = '○';
const INVERSE_WHITE_CIRCLE: char                                = '◙';
const MALE_SIGN: char                                           = '♂';
const FEMALE_SIGN: char                                         = '♀';
const EIGHTH_NOTE: char                                         = '♪';
const BEAMED_EIGHTH_NOTES: char                                 = '♫';
const WHITE_SUN_WITH_RAYS: char                                 = '☼';

// 10-1F
const BLACK_RIGHT_POINTING_POINTER: char                        = '►';
const BLACK_LEFT_POINTING_POINTER: char                         = '◄';
const UP_DOWN_ARROW: char                                       = '↕';
const DOUBLE_EXCLAMATION_MARK: char                             = '‼';
const PILCROW_SIGN: char                                        = '¶';
const SECTION_SIGN: char                                        = '§';
const BLACK_RECTANGLE: char                                     = '▬';
const UP_DOWN_ARROW_WITH_BASE: char                             = '↨';
const UPWARDS_ARROW: char                                       = '↑';
const DOWNWARDS_ARROW: char                                     = '↓';
const RIGHTWARDS_ARROW: char                                    = '→';
const LEFTWARDS_ARROW: char                                     = '←';
const RIGHT_ANGLE: char                                         = '∟';
const LEFT_RIGHT_ARROW: char                                    = '↔';
const BLACK_UP_POINTING_POINTER: char                           = '▲';
const BLACK_DOWN_POINTING_POINTER: char                         = '▼';

// 20-2F
const SPACE: char                                               = ' ';
const EXCLAMATION_MARK: char                                    = '!';
const QUOTATION_MARK: char                                      = '"';
const NUMBER_SIGN: char                                         = '#';
const DOLLAR_SIGN: char                                         = '$';
const PERCENT_SIGN: char                                        = '%';
const AMPERSAND: char                                           = '&';
const APOSTROPHE: char                                          = '\'';
const LEFT_PARENTHESIS: char                                    = '(';
const RIGHT_PARENTHESIS: char                                   = ')';
const ASTERISK: char                                            = '*';
const PLUS_SIGN: char                                           = '+';
const COMMA: char                                               = ',';
const HYPHEN_MINUS: char                                        = '-';
const FULL_STOP: char                                           = '.';
const SOLIDUS: char                                             = '/';

// 30-3F
const DIGIT_ZERO: char                                          = '0';
const DIGIT_ONE: char                                           = '1';
const DIGIT_TWO: char                                           = '2';
const DIGIT_THREE: char                                         = '3';
const DIGIT_FOUR: char                                          = '4';
const DIGIT_FIVE: char                                          = '5';
const DIGIT_SIX: char                                           = '6';
const DIGIT_SEVEN: char                                         = '7';
const DIGIT_EIGHT: char                                         = '8';
const DIGIT_NINE: char                                          = '9';
const COLON: char                                               = ':';
const SEMICOLON: char                                           = ';';
const LESS_THAN_SIGN: char                                      = '<';
const EQUALS_SIGN: char                                         = '=';
const GREATER_THAN_SIGN: char                                   = '>';
const QUESTION_MARK: char                                       = '?';

// 40-4F
const COMMERCIAL_AT: char                                       = '@';
const LATIN_CAPITAL_LETTER_A: char                              = 'A';
const LATIN_CAPITAL_LETTER_B: char                              = 'B';
const LATIN_CAPITAL_LETTER_C: char                              = 'C';
const LATIN_CAPITAL_LETTER_D: char                              = 'D';
const LATIN_CAPITAL_LETTER_E: char                              = 'E';
const LATIN_CAPITAL_LETTER_F: char                              = 'F';
const LATIN_CAPITAL_LETTER_G: char                              = 'G';
const LATIN_CAPITAL_LETTER_H: char                              = 'H';
const LATIN_CAPITAL_LETTER_I: char                              = 'I';
const LATIN_CAPITAL_LETTER_J: char                              = 'J';
const LATIN_CAPITAL_LETTER_K: char                              = 'K';
const LATIN_CAPITAL_LETTER_L: char                              = 'L';
const LATIN_CAPITAL_LETTER_M: char                              = 'M';
const LATIN_CAPITAL_LETTER_N: char                              = 'N';
const LATIN_CAPITAL_LETTER_O: char                              = 'O';

// 50-5F
const LATIN_CAPITAL_LETTER_P: char                              = 'P';
const LATIN_CAPITAL_LETTER_Q: char                              = 'Q';
const LATIN_CAPITAL_LETTER_R: char                              = 'R';
const LATIN_CAPITAL_LETTER_S: char                              = 'S';
const LATIN_CAPITAL_LETTER_T: char                              = 'T';
const LATIN_CAPITAL_LETTER_U: char                              = 'U';
const LATIN_CAPITAL_LETTER_V: char                              = 'V';
const LATIN_CAPITAL_LETTER_W: char                              = 'W';
const LATIN_CAPITAL_LETTER_X: char                              = 'X';
const LATIN_CAPITAL_LETTER_Y: char                              = 'Y';
const LATIN_CAPITAL_LETTER_Z: char                              = 'Z';
const LEFT_SQUARE_BRACKET: char                                 = '[';
const REVERSE_SOLIDUS: char                                     = '\\';
const RIGHT_SQUARE_BRACKET: char                                = ']';
const CIRCUMFLEX_ACCENT: char                                   = '^';
const LOW_LINE: char                                            = '_';

// 60-6F
const GRAVE_ACCENT: char                                        = '`';
const LATIN_SMALL_LETTER_A: char                                = 'a';
const LATIN_SMALL_LETTER_B: char                                = 'b';
const LATIN_SMALL_LETTER_C: char                                = 'c';
const LATIN_SMALL_LETTER_D: char                                = 'd';
const LATIN_SMALL_LETTER_E: char                                = 'e';
const LATIN_SMALL_LETTER_F: char                                = 'f';
const LATIN_SMALL_LETTER_G: char                                = 'g';
const LATIN_SMALL_LETTER_H: char                                = 'h';
const LATIN_SMALL_LETTER_I: char                                = 'i';
const LATIN_SMALL_LETTER_J: char                                = 'j';
const LATIN_SMALL_LETTER_K: char                                = 'k';
const LATIN_SMALL_LETTER_L: char                                = 'l';
const LATIN_SMALL_LETTER_M: char                                = 'm';
const LATIN_SMALL_LETTER_N: char                                = 'n';
const LATIN_SMALL_LETTER_O: char                                = 'o';

// 70-7F
const LATIN_SMALL_LETTER_P: char                                = 'p';
const LATIN_SMALL_LETTER_Q: char                                = 'q';
const LATIN_SMALL_LETTER_R: char                                = 'r';
const LATIN_SMALL_LETTER_S: char                                = 's';
const LATIN_SMALL_LETTER_T: char                                = 't';
const LATIN_SMALL_LETTER_U: char                                = 'u';
const LATIN_SMALL_LETTER_V: char                                = 'v';
const LATIN_SMALL_LETTER_W: char                                = 'w';
const LATIN_SMALL_LETTER_X: char                                = 'x';
const LATIN_SMALL_LETTER_Y: char                                = 'y';
const LATIN_SMALL_LETTER_Z: char                                = 'z';
const LEFT_CURLY_BRACKET: char                                  = '{';
const VERTICAL_LINE: char                                       = '|';
const RIGHT_CURLY_BRACKET: char                                 = '}';
const TILDE: char                                               = '~';
const HOUSE: char                                               = '⌂';

// 80-8F
const LATIN_CAPITAL_LETTER_C_WITH_CEDILLA: char                 = 'Ç';
const LATIN_SMALL_LETTER_U_WITH_DIAERESIS: char                 = 'ü';
const LATIN_SMALL_LETTER_E_WITH_ACUTE: char                     = 'é';
const LATIN_SMALL_LETTER_A_WITH_CIRCUMFLEX: char                = 'â';
const LATIN_SMALL_LETTER_A_WITH_DIAERESIS: char                 = 'ä';
const LATIN_SMALL_LETTER_A_WITH_GRAVE: char                     = 'à';
const LATIN_SMALL_LETTER_A_WITH_RING_ABOVE: char                = 'å';
const LATIN_SMALL_LETTER_C_WITH_CEDILLA: char                   = 'ç';
const LATIN_SMALL_LETTER_E_WITH_CIRCUMFLEX: char                = 'ê';
const LATIN_SMALL_LETTER_E_WITH_DIAERESIS: char                 = 'ë';
const LATIN_SMALL_LETTER_E_WITH_GRAVE: char                     = 'è';
const LATIN_SMALL_LETTER_I_WITH_DIAERESIS: char                 = 'ï';
const LATIN_SMALL_LETTER_I_WITH_CIRCUMFLEX: char                = 'î';
const LATIN_SMALL_LETTER_I_WITH_GRAVE: char                     = 'ì';
const LATIN_CAPITAL_LETTER_A_WITH_DIAERESIS: char               = 'Ä';
const LATIN_CAPITAL_LETTER_A_WITH_RING_ABOVE: char              = 'Å';

// 90-9F
const LATIN_CAPITAL_LETTER_E_WITH_ACUTE: char                   = 'É';
const LATIN_SMALL_LETTER_AE: char                               = 'æ';
const LATIN_CAPITAL_LETTER_AE: char                             = 'Æ';
const LATIN_SMALL_LETTER_O_WITH_CIRCUMFLEX: char                = 'ô';
const LATIN_SMALL_LETTER_O_WITH_DIAERESIS: char                 = 'ö';
const LATIN_SMALL_LETTER_O_WITH_GRAVE: char                     = 'ò';
const LATIN_SMALL_LETTER_U_WITH_CIRCUMFLEX: char                = 'û';
const LATIN_SMALL_LETTER_U_WITH_GRAVE: char                     = 'ù';
const LATIN_SMALL_LETTER_Y_WITH_DIAERESIS: char                 = 'ÿ';
const LATIN_CAPITAL_LETTER_O_WITH_DIAERESIS: char               = 'Ö';
const LATIN_CAPITAL_LETTER_U_WITH_DIAERESIS: char               = 'Ü';
const CENT_SIGN: char                                           = '¢';
const POUND_SIGN: char                                          = '£';
const YEN_SIGN: char                                            = '¥';
const PESETA_SIGN: char                                         = '₧';
const LATIN_SMALL_LETTER_F_WITH_HOOK: char                      = 'ƒ';

// A0-AF
const LATIN_SMALL_LETTER_A_WITH_ACUTE: char                     = 'á';
const LATIN_SMALL_LETTER_I_WITH_ACUTE: char                     = 'í';
const LATIN_SMALL_LETTER_O_WITH_ACUTE: char                     = 'ó';
const LATIN_SMALL_LETTER_U_WITH_ACUTE: char                     = 'ú';
const LATIN_SMALL_LETTER_N_WITH_TILDE: char                     = 'ñ';
const LATIN_CAPITAL_LETTER_N_WITH_TILDE: char                   = 'Ñ';
const FEMININE_ORDINAL_INDICATOR: char                          = 'ª';
const MASCULINE_ORDINAL_INDICATOR: char                         = 'º';
const INVERTED_QUESTION_MARK: char                              = '¿';
const REVERSED_NOT_SIGN: char                                   = '⌐';
const NOT_SIGN: char                                            = '¬';
const VULGAR_FRACTION_ONE_HALF: char                            = '½';
const VULGAR_FRACTION_ONE_QUARTER: char                         = '¼';
const INVERTED_EXCLAMATION_MARK: char                           = '¡';
const LEFT_POINTING_DOUBLE_ANGLE_QUOTATION_MARK: char           = '«';
const RIGHT_POINTING_DOUBLE_ANGLE_QUOTATION_MARK: char          = '»';

// B0-BF
const LIGHT_SHADE: char                                         = '░';
const MEDIUM_SHADE: char                                        = '▒';
const DARK_SHADE: char                                          = '▓';
const BOX_DRAWINGS_LIGHT_VERTICAL: char                         = '│';
const BOX_DRAWINGS_LIGHT_VERTICAL_AND_LEFT: char                = '┤';
const BOX_DRAWINGS_VERTICAL_SINGLE_AND_LEFT_DOUBLE: char        = '╡';
const BOX_DRAWINGS_VERTICAL_DOUBLE_AND_LEFT_SINGLE: char        = '╢';
const BOX_DRAWINGS_DOWN_DOUBLE_AND_LEFT_SINGLE: char            = '╖';
const BOX_DRAWINGS_DOWN_SINGLE_AND_LEFT_DOUBLE: char            = '╕';
const BOX_DRAWINGS_DOUBLE_VERTICAL_AND_LEFT: char               = '╣';
const BOX_DRAWINGS_DOUBLE_VERTICAL: char                        = '║';
const BOX_DRAWINGS_DOUBLE_DOWN_AND_LEFT: char                   = '╗';
const BOX_DRAWINGS_DOUBLE_UP_AND_LEFT: char                     = '╝';
const BOX_DRAWINGS_UP_DOUBLE_AND_LEFT_SINGLE: char              = '╜';
const BOX_DRAWINGS_UP_SINGLE_AND_LEFT_DOUBLE: char              = '╛';
const BOX_DRAWINGS_LIGHT_DOWN_AND_LEFT: char                    = '┐';

// C0-CF
const BOX_DRAWINGS_LIGHT_UP_AND_RIGHT: char                     = '└';
const BOX_DRAWINGS_LIGHT_UP_AND_HORIZONTAL: char                = '┴';
const BOX_DRAWINGS_LIGHT_DOWN_AND_HORIZONTAL: char              = '┬';
const BOX_DRAWINGS_LIGHT_VERTICAL_AND_RIGHT: char               = '├';
const BOX_DRAWINGS_LIGHT_HORIZONTAL: char                       = '─';
const BOX_DRAWINGS_LIGHT_VERTICAL_AND_HORIZONTAL: char          = '┼';
const BOX_DRAWINGS_VERTICAL_SINGLE_AND_RIGHT_DOUBLE: char       = '╞';
const BOX_DRAWINGS_VERTICAL_DOUBLE_AND_RIGHT_SINGLE: char       = '╟';
const BOX_DRAWINGS_DOUBLE_UP_AND_RIGHT: char                    = '╚';
const BOX_DRAWINGS_DOUBLE_DOWN_AND_RIGHT: char                  = '╔';
const BOX_DRAWINGS_DOUBLE_UP_AND_HORIZONTAL: char               = '╩';
const BOX_DRAWINGS_DOUBLE_DOWN_AND_HORIZONTAL: char             = '╦';
const BOX_DRAWINGS_DOUBLE_VERTICAL_AND_RIGHT: char              = '╠';
const BOX_DRAWINGS_DOUBLE_HORIZONTAL: char                      = '═';
const BOX_DRAWINGS_DOUBLE_VERTICAL_AND_HORIZONTAL: char         = '╬';
const BOX_DRAWINGS_UP_SINGLE_AND_HORIZONTAL_DOUBLE: char        = '╧';

// D0-DF
const BOX_DRAWINGS_UP_DOUBLE_AND_HORIZONTAL_SINGLE: char        = '╨';
const BOX_DRAWINGS_DOWN_SINGLE_AND_HORIZONTAL_DOUBLE: char      = '╤';
const BOX_DRAWINGS_DOWN_DOUBLE_AND_HORIZONTAL_SINGLE: char      = '╥';
const BOX_DRAWINGS_UP_DOUBLE_AND_RIGHT_SINGLE: char             = '╙';
const BOX_DRAWINGS_UP_SINGLE_AND_RIGHT_DOUBLE: char             = '╘';
const BOX_DRAWINGS_DOWN_SINGLE_AND_RIGHT_DOUBLE: char           = '╒';
const BOX_DRAWINGS_DOWN_DOUBLE_AND_RIGHT_SINGLE: char           = '╓';
const BOX_DRAWINGS_VERTICAL_DOUBLE_AND_HORIZONTAL_SINGLE: char  = '╫';
const BOX_DRAWINGS_VERTICAL_SINGLE_AND_HORIZONTAL_DOUBLE: char  = '╪';
const BOX_DRAWINGS_LIGHT_UP_AND_LEFT: char                      = '┘';
const BOX_DRAWINGS_LIGHT_DOWN_AND_RIGHT: char                   = '┌';
const FULL_BLOCK: char                                          = '█';
const LOWER_HALF_BLOCK: char                                    = '▄';
const LEFT_HALF_BLOCK: char                                     = '▌';
const RIGHT_HALF_BLOCK: char                                    = '▐';
const UPPER_HALF_BLOCK: char                                    = '▀';

// E0-EF
const GREEK_SMALL_LETTER_ALPHA: char                            = 'α';
const LATIN_SMALL_LETTER_SHARP_S: char                          = 'ß';
const GREEK_CAPITAL_LETTER_GAMMA: char                          = 'Γ';
const GREEK_SMALL_LETTER_PI: char                               = 'π';
const GREEK_CAPITAL_LETTER_SIGMA: char                          = 'Σ';
const GREEK_SMALL_LETTER_SIGMA: char                            = 'σ';
const MICRO_SIGN: char                                          = 'µ';
const GREEK_SMALL_LETTER_TAU: char                              = 'τ';
const GREEK_CAPITAL_LETTER_PHI: char                            = 'Φ';
const GREEK_CAPITAL_LETTER_THETA: char                          = 'Θ';
const GREEK_CAPITAL_LETTER_OMEGA: char                          = 'Ω';
const GREEK_SMALL_LETTER_DELTA: char                            = 'δ';
const INFINITY: char                                            = '∞';
const GREEK_SMALL_LETTER_PHI: char                              = 'φ';
const GREEK_SMALL_LETTER_EPSILON: char                          = 'ε';
const INTERSECTION: char                                        = '∩';

// F0-FF
const IDENTICAL_TO: char                                        = '≡';
const PLUS_MINUS_SIGN: char                                     = '±';
const GREATER_THAN_OR_EQUAL_TO: char                            = '≥';
const LESS_THAN_OR_EQUAL_TO: char                               = '≤';
const TOP_HALF_INTEGRAL: char                                   = '⌠';
const BOTTOM_HALF_INTEGRAL: char                                = '⌡';
const DIVISION_SIGN: char                                       = '÷';
const ALMOST_EQUAL_TO: char                                     = '≈';
const DEGREE_SIGN: char                                         = '°';
const BULLET_OPERATOR: char                                     = '∙';
const MIDDLE_DOT: char                                          = '·';
const SQUARE_ROOT: char                                         = '√';
const SUPERSCRIPT_LATIN_SMALL_LETTER_N: char                    = 'ⁿ';
const SUPERSCRIPT_TWO: char                                     = '²';
const BLACK_SQUARE: char                                        = '■';
const NO_BREAK_SPACE: char                                      = ' ';