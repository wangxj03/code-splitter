## Split md

```
cargo run --example split_md
```

```
0 [0..78]: 350
├─ section                          [0..2]: 4
│  ├─ atx_heading                      [2..3]: 4
│  ├─ paragraph                        [4..5]: 1
│  ├─ paragraph                        [6..8]: 8
│  ├─ html_block                       [9..12]: 2
│  ├─ paragraph                        [13..14]: 3
│  ├─ paragraph                        [15..19]: 27
│  ├─ paragraph                        [20..21]: 2
│  │  ├─ atx_heading                      [22..23]: 3
│  │  ├─ paragraph                        [24..27]: 27
│  │  ├─ list                             [28..36]: 57
│  │  ├─ link_reference_definition        [36..37]: 2
│  │  ├─ link_reference_definition        [37..38]: 2
│  │  ├─ link_reference_definition        [38..39]: 2
│  │  ├─ link_reference_definition        [39..40]: 2
│  │  ├─ link_reference_definition        [40..41]: 2
│  │  ├─ paragraph                        [42..43]: 2
│  │  │  ├─ atx_heading                      [44..45]: 2
│  │  │  ├─ paragraph                        [46..47]: 15
│  │  │  ├─ paragraph                        [48..50]: 13
│  │  │  ├─ paragraph                        [51..54]: 38
│  │  │  ├─ list                             [55..58]: 16
│  │  │  ├─ paragraph                        [58..59]: 2
│  │  │  ├─ section                          [60..78]: 114

1 [78..128]: 383
│  │  │  ├─ section                          [78..128]: 383

2 [128..182]: 494
│  │  │  ├─ atx_heading                      [128..129]: 2
│  │  │  ├─ paragraph                        [130..131]: 13
│  │  │  ├─ paragraph                        [132..135]: 27
│  │  │  ├─ list                             [136..148]: 106
│  │  │  ├─ paragraph                        [148..153]: 69
│  │  │  ├─ paragraph                        [154..162]: 85
│  │  │  ├─ paragraph                        [163..176]: 154
│  │  │  ├─ paragraph                        [177..180]: 36
│  │  │  ├─ paragraph                        [181..182]: 2

3 [183..265]: 508
│  │  │  ├─ section                          [183..211]: 228
│  │  ├─ section                          [211..265]: 280

4 [265..338]: 455
│  │  │  ├─ atx_heading                      [265..266]: 2
│  │  │  ├─ paragraph                        [267..270]: 40
│  │  │  ├─ paragraph                        [271..272]: 2
│  │  │  ├─ list                             [273..282]: 89
│  │  │  ├─ paragraph                        [282..291]: 102
│  │  │  ├─ paragraph                        [292..295]: 38
│  │  │  ├─ fenced_code_block                [296..303]: 29
│  │  │  ├─ fenced_code_block                [304..309]: 31
│  │  │  ├─ paragraph                        [310..313]: 28
│  │  │  ├─ fenced_code_block                [314..322]: 23
│  │  │  ├─ fenced_code_block                [323..334]: 41
│  │  │  ├─ paragraph                        [335..338]: 30

5 [339..375]: 281
│  │  │  ├─ paragraph                        [339..345]: 65
│  │  │  ├─ paragraph                        [346..350]: 45
│  │  │  ├─ paragraph                        [351..355]: 54
│  │  │  ├─ paragraph                        [356..357]: 2
│  │  ├─ section                          [358..375]: 115

6 [375..448]: 342
│  │  ├─ atx_heading                      [375..376]: 3
│  │  ├─ paragraph                        [377..381]: 50
│  │  ├─ paragraph                        [382..384]: 22
│  │  ├─ link_reference_definition        [385..386]: 3
│  │  ├─ paragraph                        [387..388]: 2
│  │  ├─ section                          [389..401]: 54
│  │  ├─ section                          [401..419]: 65
│  │  ├─ section                          [419..433]: 77
│  │  ├─ section                          [433..448]: 66

7 [448..483]: 251
│  │  ├─ section                          [448..483]: 251
```

## Split python

```
cargo run --example split_python_tiktoken --features tiktoken-rs
```

```
0 [0..52]: 355
├─ comment                          [0..0]: 31
├─ future_import_statement          [1..1]: 6
├─ import_statement                 [3..3]: 2
├─ import_statement                 [4..4]: 2
├─ import_from_statement            [5..5]: 6
├─ import_from_statement            [6..6]: 20
├─ import_from_statement            [7..7]: 19
├─ import_statement                 [9..9]: 3
├─ import_from_statement            [11..11]: 12
├─ import_from_statement            [12..12]: 6
├─ if_statement                     [14..15]: 16
├─ expression_statement             [18..18]: 7
│  ├─ class                            [21..21]: 1
│  ├─ identifier                       [21..21]: 1
│  ├─ argument_list                    [21..21]: 5
│  ├─ :                                [21..21]: 1
│  │  ├─ expression_statement             [22..22]: 13
│  │  ├─ expression_statemen## Split python

```

cargo run --example split_python_tiktoken

```

2 [125..158]: 249
│  ├─ class                            [125..125]: 1
│  ├─ identifier                       [125..125]: 2
│  ├─ argument_list                    [125..125]: 5
│  ├─ :                                [125..125]: 1
│  │  ├─ expression_statement             [126..126]: 13
│  │  ├─ expression_statement             [128..128]: 5
│  │  ├─ expression_statement             [130..130]: 8
│  │  ├─ function_definition              [132..143]: 79
│  │  ├─ function_definition              [145..146]: 22
│  │  ├─ function_definition              [148..150]: 26
│  │  ├─ function_definition              [152..158]: 77

3 [160..228]: 440
│  │  ├─ function_definition              [160..209]: 333
│  │  ├─ function_definition              [211..212]: 14
│  │  ├─ function_definition              [214..220]: 49
│  │  ├─ function_definition              [222..228]: 40

4 [231..298]: 452
├─ class_definition                 [231..269]: 231
│  ├─ class                            [272..272]: 1
│  ├─ identifier                       [272..272]: 3
│  ├─ :                                [272..272]: 1
│  │  ├─ expression_statement             [273..273]: 5
│  │  ├─ expression_statement             [274..274]: 5
│  │  ├─ expression_statement             [275..275]: 5
│  │  ├─ expression_statement             [276..276]: 7
│  │  ├─ function_definition              [278..282]: 37
│  │  ├─ function_definition              [284..290]: 68
│  │  ├─ function_definition              [292..298]: 74

5 [300..363]: 483
│  │  ├─ function_definition              [300..346]: 303
├─ decorated_definition             [349..357]: 98
├─ function_definition              [360..363]: 80

6 [366..388]: 146
├─ function_definition              [366..388]: 146
```

## Split golang

```
cargo run --example split_go
```

```
0 [0..149]: 362
├─ comment                          [0..0]: 13
├─ package_clause                   [1..1]: 2
├─ import_declaration               [3..14]: 12
├─ type_declaration                 [16..21]: 17
├─ type_declaration                 [23..25]: 8
├─ type_declaration                 [27..30]: 11
├─ type_declaration                 [32..36]: 14
├─ type_declaration                 [38..42]: 14
├─ type_declaration                 [44..48]: 14
├─ type_declaration                 [50..52]: 8
├─ type_declaration                 [54..66]: 38
├─ type_declaration                 [68..76]: 26
├─ type_declaration                 [78..85]: 23
├─ function_declaration             [87..99]: 34
├─ function_declaration             [101..126]: 78
├─ function_declaration             [128..149]: 50

1 [151..321]: 435
├─ function_declaration             [151..215]: 177
├─ type_declaration                 [217..221]: 10
├─ method_declaration               [223..237]: 45
├─ method_declaration               [239..278]: 105
├─ method_declaration               [280..287]: 23
├─ function_declaration             [289..321]: 75
```

## Split rust

```
cargo run --example split_rust
```

```
0 [0..100]: 423
├─ inner_attribute_item             [0..0]: 3
├─ inner_attribute_item             [1..1]: 3
├─ use_declaration                  [3..8]: 12
├─ use_declaration                  [10..10]: 2
├─ use_declaration                  [11..11]: 2
├─ mod_item                         [13..13]: 2
├─ attribute_item                   [14..14]: 3
├─ mod_item                         [15..15]: 2
├─ attribute_item                   [16..16]: 3
├─ mod_item                         [17..17]: 2
├─ mod_item                         [18..18]: 2
├─ attribute_item                   [19..19]: 3
├─ mod_item                         [20..20]: 2
├─ use_declaration                  [22..22]: 3
├─ attribute_item                   [23..23]: 3
├─ use_declaration                  [24..24]: 3
├─ use_declaration                  [25..25]: 3
├─ line_comment                     [27..28]: 14
├─ line_comment                     [28..29]: 16
├─ line_comment                     [29..30]: 7
├─ attribute_item                   [30..30]: 4
├─ struct_item                      [31..38]: 46
├─ impl_item                        [40..80]: 141
├─ line_comment                     [82..83]: 8
├─ trait_item                       [83..86]: 26
├─ line_comment                     [88..89]: 11
├─ line_comment                     [89..90]: 1
├─ line_comment                     [90..91]: 14
├─ line_comment                     [91..92]: 4
├─ line_comment                     [92..93]: 1
├─ line_comment                     [93..94]: 14
├─ line_comment                     [94..95]: 15
├─ line_comment                     [95..96]: 11
├─ line_comment                     [96..97]: 1
├─ line_comment                     [97..98]: 16
├─ line_comment                     [98..99]: 15
├─ line_comment                     [99..100]: 5

1 [100..227]: 405
├─ trait_item                       [100..132]: 133
├─ impl_item                        [134..138]: 13
├─ impl_item                        [140..148]: 20
├─ impl_item                        [150..158]: 20
├─ impl_item                        [160..168]: 20
├─ impl_item                        [170..178]: 20
├─ impl_item                        [180..188]: 20
├─ impl_item                        [190..198]: 20
├─ line_comment                     [200..201]: 11
├─ attribute_item                   [201..201]: 1
├─ attribute_item                   [202..202]: 7
├─ enum_item                        [203..210]: 40
├─ line_comment                     [212..213]: 7
├─ trait_item                       [213..220]: 29
├─ line_comment                     [222..223]: 9
├─ line_comment                     [223..224]: 13
├─ line_comment                     [224..225]: 5
├─ line_comment                     [225..226]: 12
├─ line_comment                     [226..227]: 5

2 [227..318]: 415
├─ trait_item                       [227..290]: 310
├─ line_comment                     [292..293]: 12
├─ attribute_item                   [293..293]: 1
├─ struct_item                      [294..312]: 74
│  ├─ impl                             [314..314]: 1
│  ├─ type_parameters                  [314..314]: 6
│  ├─ generic_type                     [314..314]: 5
│  ├─ where_clause                     [315..318]: 7

3 [319..453]: 511
│  │  ├─ {                                [319..319]: 1
│  │  ├─ line_comment                     [320..321]: 9
│  │  ├─ line_comment                     [321..322]: 7
│  │  ├─ function_item                    [322..331]: 26
│  │  ├─ line_comment                     [333..334]: 13
│  │  ├─ function_item                    [334..340]: 24
│  │  ├─ line_comment                     [342..343]: 9
│  │  ├─ function_item                    [343..350]: 33
│  │  ├─ line_comment                     [352..353]: 8
│  │  ├─ line_comment                     [353..354]: 7
│  │  ├─ line_comment                     [354..355]: 9
│  │  ├─ function_item                    [355..450]: 350
│  │  ├─ line_comment                     [452..453]: 15

4 [453..527]: 364
│  │  ├─ line_comment                     [453..454]: 14
│  │  ├─ line_comment                     [454..455]: 4
│  │  ├─ function_item                    [455..487]: 223
│  │  ├─ function_item                    [489..498]: 25
│  │  ├─ }                                [499..499]: 1
├─ impl_item                        [501..524]: 84
├─ line_comment                     [526..527]: 13

5 [527..725]: 447
├─ function_item                    [527..600]: 222
├─ attribute_item                   [602..602]: 1
├─ mod_item                         [603..725]: 224
```
