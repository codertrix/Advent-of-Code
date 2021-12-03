let input = """
dotted salmon bags contain 2 dark lavender bags, 1 muted red bag, 1 vibrant magenta bag.
vibrant purple bags contain 1 pale cyan bag, 1 dotted lavender bag, 3 striped blue bags, 5 clear magenta bags.
vibrant fuchsia bags contain 4 posh violet bags, 3 bright aqua bags, 1 light silver bag.
mirrored purple bags contain 2 dim yellow bags, 4 dim green bags, 3 vibrant beige bags.
faded coral bags contain 1 vibrant plum bag, 3 pale gold bags, 5 dim purple bags, 1 drab teal bag.
wavy cyan bags contain 4 dark teal bags, 1 dotted magenta bag.
dotted gold bags contain 3 dotted gray bags.
shiny maroon bags contain 2 light white bags, 5 bright salmon bags.
vibrant cyan bags contain 2 dull beige bags.
clear fuchsia bags contain 5 bright bronze bags.
clear violet bags contain 4 clear white bags, 2 drab fuchsia bags, 4 plaid lavender bags, 4 drab beige bags.
wavy tomato bags contain 3 wavy yellow bags, 4 plaid maroon bags, 1 dark beige bag.
shiny indigo bags contain 4 posh aqua bags, 2 dim salmon bags, 3 dotted olive bags, 1 dull gray bag.
dotted indigo bags contain 3 muted bronze bags, 1 striped salmon bag, 5 vibrant violet bags, 3 drab crimson bags.
mirrored magenta bags contain 1 shiny aqua bag.
pale green bags contain 4 striped orange bags, 3 plaid red bags, 3 clear olive bags.
dim crimson bags contain 5 posh violet bags, 3 mirrored crimson bags, 2 striped white bags, 1 dark indigo bag.
light white bags contain 1 dark indigo bag, 1 wavy orange bag.
striped green bags contain 2 clear beige bags, 4 dim coral bags.
dotted lavender bags contain 1 striped white bag, 5 dotted coral bags, 3 striped orange bags, 1 dotted gray bag.
light purple bags contain 5 dim tomato bags, 4 dull plum bags, 1 dim green bag, 3 dotted magenta bags.
shiny violet bags contain 1 shiny white bag.
dark lime bags contain 4 mirrored crimson bags, 2 wavy crimson bags, 4 dim green bags.
dark teal bags contain 3 mirrored orange bags.
muted maroon bags contain 5 dim fuchsia bags.
posh tomato bags contain 3 bright violet bags, 3 dim crimson bags, 4 mirrored beige bags, 3 drab lime bags.
plaid violet bags contain 1 shiny tomato bag, 5 dotted gray bags, 1 muted magenta bag, 5 mirrored crimson bags.
shiny bronze bags contain 3 dim red bags, 2 drab plum bags, 3 striped yellow bags, 1 mirrored yellow bag.
muted tomato bags contain 2 vibrant maroon bags, 3 vibrant beige bags, 5 light coral bags.
muted fuchsia bags contain 2 clear violet bags, 4 shiny gray bags, 1 shiny gold bag, 3 wavy crimson bags.
pale teal bags contain 2 bright beige bags, 3 vibrant cyan bags, 4 bright salmon bags, 3 plaid red bags.
posh salmon bags contain 2 dull beige bags, 1 faded maroon bag.
dim white bags contain 2 dull tan bags, 3 dotted coral bags, 1 light silver bag, 1 dim gold bag.
posh tan bags contain 4 clear blue bags, 4 dim olive bags, 4 striped green bags.
dim lime bags contain 2 drab chartreuse bags, 5 striped magenta bags, 4 striped purple bags.
mirrored white bags contain 2 dotted white bags, 3 striped salmon bags, 3 dull green bags, 3 plaid orange bags.
shiny fuchsia bags contain 3 striped bronze bags, 1 plaid violet bag.
dull indigo bags contain 5 wavy tomato bags.
posh maroon bags contain 2 dark green bags, 4 posh red bags, 3 drab tomato bags.
clear coral bags contain 5 pale yellow bags, 2 muted green bags, 3 faded black bags, 1 striped teal bag.
posh gold bags contain 3 wavy yellow bags, 5 mirrored crimson bags, 1 dotted lavender bag, 2 plaid purple bags.
muted yellow bags contain 3 wavy tan bags, 1 dark beige bag, 3 drab lavender bags.
pale cyan bags contain 5 clear black bags, 3 dull lime bags, 3 wavy olive bags, 5 plaid indigo bags.
drab white bags contain 2 dull green bags, 3 wavy aqua bags, 4 dark indigo bags.
drab lime bags contain 2 drab aqua bags, 5 light turquoise bags.
dotted tan bags contain 1 dotted beige bag, 5 faded maroon bags, 5 light indigo bags, 3 light bronze bags.
muted crimson bags contain 5 faded gold bags.
striped turquoise bags contain 1 shiny silver bag, 4 vibrant salmon bags, 5 dotted yellow bags.
plaid yellow bags contain 2 dim crimson bags, 5 bright tan bags, 3 wavy yellow bags, 1 clear silver bag.
shiny salmon bags contain 2 vibrant yellow bags.
pale maroon bags contain 5 faded silver bags, 3 dull lavender bags, 4 dotted magenta bags.
dotted orange bags contain 4 mirrored brown bags, 2 clear fuchsia bags.
muted gray bags contain 5 dim purple bags, 3 wavy yellow bags.
vibrant gold bags contain 2 wavy maroon bags, 5 faded chartreuse bags, 3 light indigo bags, 4 dark brown bags.
bright fuchsia bags contain 4 pale crimson bags, 3 striped orange bags, 1 shiny tomato bag, 5 bright tan bags.
light cyan bags contain 1 dim teal bag, 3 wavy indigo bags.
pale olive bags contain 3 light fuchsia bags.
dim brown bags contain 2 dotted indigo bags.
mirrored silver bags contain 3 plaid violet bags.
posh cyan bags contain 5 wavy maroon bags, 4 dim gold bags.
bright turquoise bags contain 5 posh bronze bags, 4 shiny aqua bags.
faded fuchsia bags contain 3 dotted gray bags.
dull beige bags contain 3 posh bronze bags.
dark purple bags contain 4 dim salmon bags, 4 faded maroon bags, 2 drab red bags, 1 clear bronze bag.
drab tan bags contain 5 striped maroon bags.
faded green bags contain 3 dark lavender bags, 4 posh lime bags, 2 light purple bags, 2 dark plum bags.
posh indigo bags contain 4 dark tan bags, 2 dark lavender bags, 1 shiny cyan bag.
drab maroon bags contain 3 wavy red bags, 1 dim fuchsia bag, 5 mirrored indigo bags, 2 drab lavender bags.
dim magenta bags contain 4 striped orange bags.
striped teal bags contain 2 dark lime bags.
plaid green bags contain 5 mirrored salmon bags, 4 mirrored brown bags, 2 dark lavender bags, 4 faded indigo bags.
dull blue bags contain 3 faded lime bags, 2 faded violet bags, 4 dull tan bags, 1 shiny tan bag.
muted turquoise bags contain 5 dim silver bags, 4 wavy crimson bags.
dull teal bags contain 1 faded blue bag, 3 light violet bags, 3 faded black bags.
mirrored salmon bags contain 4 muted magenta bags, 3 dotted tomato bags, 2 light cyan bags, 2 vibrant lavender bags.
shiny coral bags contain 1 striped teal bag, 1 drab lime bag, 5 mirrored red bags.
muted chartreuse bags contain 2 striped white bags, 3 bright turquoise bags, 2 faded silver bags.
mirrored green bags contain 2 bright maroon bags.
light brown bags contain 2 clear bronze bags, 4 dark lime bags, 3 muted teal bags, 5 wavy yellow bags.
vibrant indigo bags contain 3 dotted bronze bags.
light coral bags contain 2 dotted bronze bags.
dotted turquoise bags contain 3 plaid red bags, 3 shiny fuchsia bags, 4 faded lime bags.
dark white bags contain 1 dull blue bag, 5 shiny cyan bags, 3 dark beige bags, 2 mirrored brown bags.
pale purple bags contain 5 faded gold bags, 2 drab maroon bags.
wavy beige bags contain 3 faded violet bags, 4 mirrored silver bags, 4 clear gray bags, 2 dotted tomato bags.
light teal bags contain 4 bright magenta bags, 2 drab coral bags.
bright tomato bags contain 3 mirrored crimson bags, 1 muted blue bag, 4 dim brown bags.
plaid turquoise bags contain 4 vibrant blue bags, 1 wavy chartreuse bag, 2 pale magenta bags.
faded tomato bags contain 3 light salmon bags, 2 wavy chartreuse bags.
faded salmon bags contain 5 dark aqua bags, 4 faded bronze bags, 5 bright crimson bags.
light salmon bags contain 2 mirrored silver bags, 3 dotted coral bags, 3 wavy crimson bags.
mirrored lavender bags contain 4 shiny silver bags, 1 wavy crimson bag.
vibrant plum bags contain 5 light orange bags, 4 dull lime bags, 2 dim aqua bags, 5 shiny violet bags.
vibrant tan bags contain 1 drab salmon bag, 2 dull beige bags, 3 dotted bronze bags.
faded red bags contain 3 light indigo bags, 2 dotted tomato bags.
bright purple bags contain 3 light tomato bags, 3 clear gold bags.
plaid magenta bags contain 5 muted orange bags, 3 pale plum bags, 5 faded plum bags.
wavy tan bags contain 1 dull lavender bag.
faded olive bags contain 5 mirrored silver bags.
plaid white bags contain 4 bright bronze bags, 2 dotted cyan bags, 2 dark lavender bags, 5 shiny lavender bags.
striped aqua bags contain 5 light salmon bags.
drab tomato bags contain 1 dark lime bag, 2 muted magenta bags, 5 clear gray bags, 3 dotted gray bags.
clear tomato bags contain 5 plaid bronze bags.
posh teal bags contain 1 plaid maroon bag, 3 light tan bags, 1 clear crimson bag, 5 vibrant aqua bags.
dim tan bags contain 2 drab fuchsia bags, 3 dark beige bags, 3 plaid green bags.
dotted lime bags contain 4 dim red bags.
mirrored beige bags contain 1 bright orange bag, 3 light purple bags.
plaid silver bags contain 2 bright gold bags.
faded orange bags contain 5 faded plum bags.
dim indigo bags contain 4 muted orange bags, 5 vibrant tomato bags.
shiny blue bags contain 3 drab crimson bags, 2 dim green bags, 1 clear violet bag.
dull olive bags contain 2 plaid lime bags, 5 muted maroon bags, 4 shiny crimson bags, 3 dim plum bags.
striped maroon bags contain 1 dim tomato bag, 2 dotted tomato bags, 1 muted magenta bag, 5 faded maroon bags.
dull green bags contain 4 light salmon bags, 1 dim green bag.
vibrant crimson bags contain 4 clear plum bags, 2 faded white bags, 4 wavy crimson bags.
striped chartreuse bags contain 2 plaid lavender bags.
bright chartreuse bags contain 2 vibrant orange bags.
drab lavender bags contain 4 dotted coral bags.
dull crimson bags contain 2 faded silver bags, 2 bright magenta bags.
striped magenta bags contain 4 dull crimson bags, 4 faded maroon bags.
clear bronze bags contain 1 bright bronze bag, 2 shiny red bags.
plaid tan bags contain 5 pale silver bags, 5 muted teal bags, 3 faded beige bags, 2 faded chartreuse bags.
dim black bags contain 2 faded indigo bags.
shiny black bags contain 3 dotted bronze bags, 5 clear olive bags, 3 dark orange bags, 4 pale coral bags.
striped gold bags contain 3 dotted coral bags, 4 faded orange bags, 2 striped purple bags, 4 light cyan bags.
drab chartreuse bags contain 4 dull crimson bags.
bright yellow bags contain 3 wavy crimson bags, 4 bright turquoise bags, 3 plaid red bags.
dark orange bags contain 3 wavy yellow bags.
clear salmon bags contain 5 wavy turquoise bags, 4 shiny aqua bags, 1 shiny brown bag, 1 vibrant tomato bag.
plaid fuchsia bags contain 1 posh olive bag.
wavy magenta bags contain 5 vibrant beige bags, 5 faded chartreuse bags, 5 light purple bags, 4 wavy indigo bags.
dull gold bags contain 2 shiny bronze bags, 4 plaid beige bags, 4 mirrored silver bags, 2 bright lavender bags.
drab magenta bags contain 1 posh red bag.
shiny silver bags contain 4 dark lavender bags, 2 dull tan bags.
shiny crimson bags contain 3 plaid violet bags, 4 muted purple bags.
posh black bags contain 2 dotted magenta bags, 4 dotted gray bags.
vibrant salmon bags contain 1 clear silver bag.
dotted tomato bags contain 4 wavy crimson bags, 3 faded plum bags, 4 muted magenta bags.
dark turquoise bags contain 1 dim beige bag, 3 dotted coral bags.
pale orange bags contain 2 striped silver bags, 1 dotted salmon bag, 3 dim black bags, 4 posh turquoise bags.
mirrored indigo bags contain 5 posh yellow bags.
muted lavender bags contain 5 striped olive bags, 1 muted indigo bag.
light yellow bags contain 5 dark chartreuse bags.
drab bronze bags contain 3 clear beige bags, 3 bright purple bags, 3 bright brown bags.
mirrored blue bags contain 5 dotted bronze bags, 1 dark green bag, 5 clear silver bags, 5 dim maroon bags.
wavy violet bags contain 4 light tan bags, 1 vibrant lavender bag.
clear red bags contain 4 faded orange bags, 1 drab gold bag, 4 dim teal bags, 4 dotted indigo bags.
striped indigo bags contain 5 dim indigo bags.
striped olive bags contain 3 dotted green bags, 4 mirrored cyan bags.
dim fuchsia bags contain 3 vibrant gray bags, 5 wavy brown bags, 2 muted beige bags.
plaid plum bags contain 3 shiny tomato bags, 4 striped orange bags.
striped white bags contain 1 plaid red bag.
mirrored coral bags contain 5 drab teal bags, 4 dotted coral bags, 4 striped chartreuse bags, 5 dotted gold bags.
dull bronze bags contain 4 dim olive bags, 5 posh turquoise bags, 2 clear gold bags.
pale white bags contain 3 vibrant gray bags, 4 wavy coral bags, 5 drab purple bags.
drab green bags contain 1 dim teal bag, 1 faded bronze bag, 3 clear silver bags.
dotted black bags contain 3 drab tan bags, 2 vibrant orange bags, 3 striped maroon bags.
light gold bags contain 5 posh olive bags, 3 striped orange bags, 3 dull orange bags.
pale tan bags contain 1 mirrored fuchsia bag, 4 light silver bags, 5 dim plum bags.
plaid lavender bags contain 1 dim teal bag, 5 shiny aqua bags, 5 wavy crimson bags.
wavy lime bags contain 1 pale chartreuse bag, 5 mirrored white bags, 4 faded olive bags.
bright crimson bags contain 1 plaid green bag.
pale fuchsia bags contain 4 light aqua bags.
striped crimson bags contain 4 shiny plum bags, 3 dull violet bags, 5 clear tan bags.
mirrored fuchsia bags contain 3 posh red bags, 3 plaid violet bags, 5 shiny brown bags, 5 striped chartreuse bags.
faded cyan bags contain 3 striped yellow bags, 3 clear indigo bags.
mirrored plum bags contain 2 vibrant silver bags.
dull turquoise bags contain 2 plaid white bags, 5 striped salmon bags, 5 clear purple bags.
dim orange bags contain 5 dim black bags.
dull orange bags contain 3 dotted magenta bags, 2 mirrored crimson bags, 3 striped bronze bags.
dark gray bags contain 4 dotted crimson bags, 2 vibrant salmon bags.
drab orange bags contain 5 dotted gray bags, 5 muted maroon bags, 5 faded aqua bags, 2 pale maroon bags.
striped bronze bags contain 3 striped white bags, 5 striped orange bags.
light bronze bags contain 2 plaid red bags, 2 faded olive bags.
drab olive bags contain 2 striped salmon bags, 3 drab white bags, 4 pale silver bags.
bright cyan bags contain 5 muted blue bags.
drab black bags contain 1 shiny blue bag, 1 drab yellow bag, 5 muted tan bags, 2 drab violet bags.
shiny green bags contain 5 dim purple bags, 1 dotted crimson bag.
wavy blue bags contain 3 shiny white bags.
pale lime bags contain 1 dotted maroon bag.
dotted red bags contain 1 dotted bronze bag, 1 vibrant lime bag, 4 clear plum bags.
faded bronze bags contain 4 pale magenta bags, 1 faded black bag, 1 faded orange bag, 3 dark crimson bags.
dark green bags contain 1 striped magenta bag.
faded blue bags contain 2 dim maroon bags.
dim violet bags contain 4 posh yellow bags, 5 dim lime bags, 5 faded olive bags.
dark tan bags contain 2 mirrored green bags.
dim teal bags contain 5 mirrored yellow bags, 2 dull tan bags, 2 vibrant lavender bags, 4 clear lime bags.
plaid crimson bags contain 3 striped black bags, 1 faded plum bag, 3 muted red bags.
clear white bags contain 1 shiny gold bag.
dim silver bags contain 5 dull tan bags.
dark cyan bags contain 2 clear black bags, 5 plaid yellow bags, 2 posh coral bags.
dull violet bags contain 5 wavy teal bags, 5 shiny white bags.
muted coral bags contain 5 faded violet bags, 2 drab red bags, 3 muted olive bags, 4 mirrored tomato bags.
vibrant white bags contain 3 muted brown bags, 4 mirrored red bags, 4 dull orange bags, 4 dark crimson bags.
posh red bags contain 2 clear gray bags, 2 bright turquoise bags.
wavy chartreuse bags contain 5 light tomato bags, 5 posh black bags.
striped black bags contain 1 faded green bag, 1 wavy green bag, 4 wavy orange bags.
drab indigo bags contain 4 clear cyan bags, 4 mirrored crimson bags, 4 clear silver bags.
pale gold bags contain 1 dim lime bag.
dark olive bags contain 5 bright white bags, 5 clear lavender bags, 2 vibrant coral bags.
muted salmon bags contain 4 drab beige bags, 3 plaid violet bags, 4 mirrored brown bags, 3 dark lime bags.
posh gray bags contain 1 clear beige bag, 4 vibrant blue bags, 1 shiny aqua bag, 5 dim tan bags.
bright white bags contain 5 muted bronze bags, 5 clear cyan bags.
dull white bags contain 1 vibrant tomato bag.
striped tan bags contain 4 mirrored brown bags, 5 faded chartreuse bags.
wavy teal bags contain 3 plaid red bags.
faded purple bags contain 4 muted orange bags, 3 clear salmon bags, 5 drab cyan bags.
dotted yellow bags contain 4 light indigo bags, 5 drab violet bags, 3 dull tan bags, 3 bright lime bags.
wavy plum bags contain 2 dim aqua bags, 5 dark brown bags, 1 bright gold bag, 2 dull orange bags.
muted blue bags contain 5 plaid tan bags, 4 dim salmon bags, 5 striped tomato bags.
dim green bags contain no other bags.
light lime bags contain 3 mirrored orange bags, 2 muted red bags, 1 dim black bag, 4 posh maroon bags.
mirrored brown bags contain 4 dotted gray bags, 3 clear lime bags, 2 dim green bags.
drab fuchsia bags contain 5 mirrored yellow bags, 5 plaid plum bags.
drab gray bags contain 2 dim violet bags, 4 posh purple bags.
vibrant teal bags contain 4 pale tomato bags, 2 posh lime bags, 2 mirrored silver bags, 1 wavy crimson bag.
dotted plum bags contain 5 bright gold bags.
shiny gold bags contain 4 shiny tomato bags, 5 wavy indigo bags.
bright bronze bags contain 2 faded maroon bags, 2 dim white bags, 5 drab violet bags.
drab yellow bags contain 4 mirrored green bags, 5 faded fuchsia bags, 1 drab turquoise bag.
dark bronze bags contain 5 muted salmon bags, 3 posh cyan bags, 2 shiny tan bags.
light magenta bags contain 3 dark silver bags, 2 striped tan bags.
pale yellow bags contain 5 drab tomato bags, 3 striped orange bags, 4 striped maroon bags.
shiny cyan bags contain 2 light tomato bags, 3 clear purple bags, 2 shiny tan bags, 3 dull tan bags.
pale gray bags contain 2 drab purple bags, 5 dotted turquoise bags, 3 pale salmon bags.
mirrored cyan bags contain 4 faded chartreuse bags.
dotted bronze bags contain 2 faded silver bags.
faded beige bags contain 1 bright red bag, 3 dotted red bags, 2 striped lime bags.
dotted beige bags contain 4 muted turquoise bags, 1 dull beige bag, 3 bright gold bags.
drab plum bags contain 4 clear chartreuse bags.
pale blue bags contain 1 dim chartreuse bag, 1 dark lavender bag, 1 drab tan bag.
faded teal bags contain 1 vibrant tan bag, 1 dim tan bag, 3 dull gray bags, 4 plaid red bags.
plaid beige bags contain 3 plaid tomato bags.
dotted chartreuse bags contain 1 pale maroon bag.
bright gray bags contain 2 bright coral bags, 3 bright turquoise bags, 4 posh tan bags.
bright olive bags contain 1 dotted bronze bag, 4 mirrored yellow bags, 4 light bronze bags.
dotted olive bags contain 5 clear purple bags, 5 muted bronze bags, 5 vibrant tomato bags.
pale tomato bags contain 4 clear gold bags, 4 clear tomato bags.
clear gold bags contain 1 striped lime bag, 1 striped chartreuse bag, 2 muted fuchsia bags, 5 clear salmon bags.
dotted gray bags contain no other bags.
striped brown bags contain 3 pale maroon bags, 5 faded gold bags, 4 wavy yellow bags, 3 bright turquoise bags.
shiny lime bags contain 4 pale violet bags, 4 clear violet bags.
mirrored tan bags contain 1 vibrant magenta bag, 3 mirrored violet bags, 5 faded plum bags.
dull maroon bags contain 5 posh salmon bags, 5 dull coral bags, 2 plaid plum bags, 3 striped teal bags.
dim purple bags contain 1 light tomato bag, 3 shiny violet bags.
bright brown bags contain 1 shiny aqua bag, 1 shiny lavender bag, 1 drab fuchsia bag, 3 faded olive bags.
light maroon bags contain 4 mirrored brown bags.
pale violet bags contain 4 wavy turquoise bags, 5 dull orange bags, 4 dotted black bags, 5 muted fuchsia bags.
mirrored black bags contain 3 dotted silver bags, 2 bright crimson bags.
drab aqua bags contain 3 plaid red bags, 1 plaid violet bag, 5 dim tomato bags.
clear green bags contain 4 dark coral bags.
mirrored aqua bags contain 4 shiny crimson bags, 4 wavy bronze bags.
pale coral bags contain 2 wavy olive bags, 2 muted silver bags, 4 dim tan bags.
mirrored gray bags contain 5 clear plum bags, 5 dark yellow bags.
light blue bags contain 3 dull bronze bags, 3 dotted plum bags.
dark silver bags contain 2 plaid chartreuse bags, 5 plaid lavender bags, 1 wavy indigo bag, 4 dark brown bags.
posh silver bags contain 4 shiny lavender bags.
shiny orange bags contain 4 light magenta bags.
dark blue bags contain 5 pale purple bags.
wavy white bags contain 1 posh olive bag, 1 dotted fuchsia bag, 4 muted maroon bags.
striped orange bags contain 2 faded plum bags.
dull lime bags contain 1 pale yellow bag, 1 clear turquoise bag, 5 faded silver bags, 4 dim beige bags.
dull red bags contain 1 bright aqua bag, 3 light maroon bags, 5 light tomato bags.
light tomato bags contain 3 dull crimson bags, 1 dim beige bag.
bright blue bags contain 3 plaid olive bags.
bright red bags contain 4 muted teal bags, 5 dotted magenta bags.
striped gray bags contain 2 dotted purple bags, 4 dull green bags, 4 dull salmon bags, 4 muted silver bags.
drab salmon bags contain 4 mirrored indigo bags, 5 mirrored silver bags, 5 shiny cyan bags, 1 plaid brown bag.
posh olive bags contain 3 faded silver bags, 5 plaid violet bags, 2 striped bronze bags.
dim salmon bags contain 5 vibrant lavender bags.
mirrored orange bags contain 2 bright white bags, 5 plaid orange bags.
posh fuchsia bags contain 5 vibrant coral bags, 2 shiny plum bags, 5 pale silver bags.
wavy fuchsia bags contain 5 posh cyan bags, 2 bright silver bags, 1 wavy tomato bag, 1 wavy plum bag.
clear olive bags contain 4 pale maroon bags, 2 muted bronze bags, 4 mirrored crimson bags, 2 dull blue bags.
clear blue bags contain 5 shiny white bags, 5 plaid lime bags.
dotted coral bags contain no other bags.
clear black bags contain 4 dull beige bags, 2 mirrored crimson bags, 2 dim beige bags.
mirrored turquoise bags contain 2 light green bags, 4 dull lime bags, 2 drab olive bags, 3 drab purple bags.
muted teal bags contain 2 dim gold bags, 1 light salmon bag, 3 dark crimson bags, 3 muted olive bags.
clear gray bags contain 2 dotted coral bags, 5 shiny tomato bags.
vibrant black bags contain 2 dull blue bags, 1 light magenta bag.
clear yellow bags contain 3 vibrant yellow bags, 2 plaid red bags, 1 dull plum bag, 4 faded violet bags.
dull fuchsia bags contain 4 dim maroon bags, 4 wavy plum bags, 5 dim teal bags.
mirrored gold bags contain 4 mirrored brown bags, 3 dotted coral bags, 4 faded plum bags, 1 mirrored indigo bag.
faded lime bags contain 3 plaid violet bags, 2 drab tan bags.
wavy olive bags contain 3 muted silver bags, 1 pale maroon bag, 5 posh silver bags.
pale bronze bags contain 2 striped purple bags, 5 bright magenta bags, 4 pale crimson bags.
posh crimson bags contain 4 mirrored purple bags, 2 shiny silver bags, 4 bright tan bags.
clear plum bags contain 1 mirrored indigo bag, 4 clear purple bags, 2 dull blue bags, 5 striped bronze bags.
vibrant chartreuse bags contain 5 vibrant yellow bags, 1 faded red bag.
plaid red bags contain no other bags.
dim maroon bags contain 2 pale yellow bags, 4 dotted cyan bags.
pale silver bags contain 2 dark crimson bags.
bright magenta bags contain 5 plaid red bags, 4 faded maroon bags.
dark crimson bags contain 2 bright magenta bags, 1 mirrored silver bag, 2 mirrored brown bags, 1 shiny lavender bag.
dull yellow bags contain 1 dull salmon bag.
dark maroon bags contain 5 muted turquoise bags.
clear indigo bags contain 5 posh purple bags, 3 striped magenta bags.
shiny olive bags contain 3 drab yellow bags.
vibrant bronze bags contain 4 plaid indigo bags.
light plum bags contain 5 faded aqua bags.
dim lavender bags contain 3 drab purple bags, 2 clear salmon bags, 1 wavy bronze bag, 1 plaid tan bag.
mirrored chartreuse bags contain 3 wavy plum bags, 2 muted tan bags.
bright teal bags contain 2 bright lavender bags, 5 plaid plum bags, 3 clear bronze bags.
dull tan bags contain 4 clear gray bags.
shiny magenta bags contain 2 plaid gold bags, 2 dotted turquoise bags, 4 muted teal bags, 3 shiny gold bags.
clear aqua bags contain 4 dim cyan bags, 3 dim plum bags, 5 dim violet bags, 1 plaid yellow bag.
clear tan bags contain 1 shiny brown bag.
faded lavender bags contain 3 faded white bags, 1 faded purple bag.
plaid maroon bags contain 5 plaid orange bags, 4 striped green bags, 2 striped gold bags, 5 bright bronze bags.
wavy lavender bags contain 5 dim black bags, 5 clear fuchsia bags.
plaid olive bags contain 3 dotted crimson bags, 2 pale plum bags, 3 bright orange bags, 1 clear coral bag.
shiny lavender bags contain 3 faded plum bags, 2 mirrored crimson bags, 5 striped orange bags, 5 bright magenta bags.
plaid chartreuse bags contain 5 clear silver bags, 5 pale crimson bags.
bright coral bags contain 4 vibrant orange bags.
wavy yellow bags contain 3 dull plum bags, 4 bright yellow bags, 5 mirrored violet bags, 3 plaid red bags.
dark chartreuse bags contain 4 striped bronze bags, 2 bright tan bags.
mirrored tomato bags contain 1 clear yellow bag, 1 dark lime bag, 2 mirrored silver bags.
plaid gray bags contain 4 muted cyan bags, 4 dark silver bags, 1 wavy orange bag, 4 muted tomato bags.
mirrored red bags contain 5 pale crimson bags, 5 light cyan bags, 1 posh yellow bag.
vibrant silver bags contain 2 dull brown bags, 1 vibrant orange bag, 5 striped red bags.
plaid purple bags contain 2 striped maroon bags, 1 faded olive bag.
dull silver bags contain 4 shiny lavender bags, 5 plaid fuchsia bags, 1 plaid plum bag, 2 light cyan bags.
dark plum bags contain 2 vibrant gold bags, 4 plaid brown bags, 1 drab teal bag, 4 dotted yellow bags.
mirrored teal bags contain 3 drab orange bags, 4 shiny fuchsia bags, 4 mirrored lavender bags, 1 clear brown bag.
light red bags contain 5 striped violet bags.
shiny aqua bags contain 4 wavy indigo bags, 3 posh olive bags, 2 clear gray bags, 2 dim tomato bags.
dull magenta bags contain 1 bright crimson bag, 1 bright bronze bag, 2 drab gold bags, 2 clear lavender bags.
muted gold bags contain 1 striped chartreuse bag, 3 posh olive bags.
dark brown bags contain 1 bright yellow bag.
dull brown bags contain 5 dark brown bags, 1 dim gold bag.
shiny chartreuse bags contain 1 shiny aqua bag, 5 faded chartreuse bags, 4 wavy beige bags.
vibrant maroon bags contain 1 dotted yellow bag, 2 striped green bags, 2 muted olive bags, 2 muted turquoise bags.
faded maroon bags contain 1 striped bronze bag, 5 dotted coral bags, 4 dim green bags, 1 faded plum bag.
wavy salmon bags contain 1 dim white bag, 4 clear purple bags, 5 dark fuchsia bags, 5 vibrant cyan bags.
light beige bags contain 3 dull lime bags, 1 plaid lavender bag.
plaid teal bags contain 2 muted black bags, 2 dull salmon bags, 5 faded red bags, 2 muted turquoise bags.
plaid cyan bags contain 5 faded maroon bags, 2 posh chartreuse bags.
dim chartreuse bags contain 5 muted red bags, 4 dark lime bags.
clear magenta bags contain 1 clear tomato bag, 1 striped orange bag, 3 striped chartreuse bags.
dotted blue bags contain 5 plaid green bags, 3 mirrored crimson bags, 4 dotted magenta bags, 3 clear bronze bags.
clear silver bags contain 4 vibrant lavender bags, 1 wavy turquoise bag, 2 posh purple bags, 5 dull blue bags.
posh beige bags contain 4 muted aqua bags, 3 wavy orange bags, 1 muted coral bag.
dim coral bags contain 1 clear purple bag, 2 plaid purple bags, 5 light gold bags.
faded white bags contain 2 striped orange bags, 3 posh silver bags, 4 shiny tan bags, 5 faded green bags.
pale aqua bags contain 5 shiny tan bags, 5 drab violet bags.
clear brown bags contain 5 shiny tomato bags.
faded tan bags contain 1 pale gray bag, 2 dull beige bags, 3 wavy teal bags, 1 bright maroon bag.
striped silver bags contain 3 shiny green bags.
vibrant brown bags contain 5 shiny aqua bags, 2 pale violet bags, 5 drab indigo bags.
dark yellow bags contain 1 wavy white bag, 3 posh violet bags, 4 bright magenta bags.
dull gray bags contain 3 dark silver bags, 3 mirrored green bags.
dark violet bags contain 2 dark maroon bags, 5 dull aqua bags.
muted brown bags contain 5 dim aqua bags, 3 dim teal bags, 4 faded olive bags, 1 plaid purple bag.
posh chartreuse bags contain 3 dark tan bags, 1 striped salmon bag, 2 dark lime bags, 5 vibrant beige bags.
mirrored maroon bags contain 1 dark red bag.
faded yellow bags contain 5 mirrored beige bags, 1 drab chartreuse bag, 3 vibrant lavender bags.
dotted aqua bags contain 4 vibrant lavender bags, 4 shiny fuchsia bags.
bright maroon bags contain 5 faded maroon bags, 2 dark lime bags, 4 dim beige bags.
dark salmon bags contain 5 clear tan bags.
wavy bronze bags contain 3 dark beige bags.
wavy crimson bags contain no other bags.
drab gold bags contain 2 faded violet bags, 4 faded silver bags.
dark black bags contain 3 wavy bronze bags, 2 dark aqua bags, 4 dotted beige bags.
striped lavender bags contain 2 wavy aqua bags.
vibrant red bags contain 1 plaid aqua bag.
shiny yellow bags contain 5 clear silver bags, 2 dull magenta bags, 5 clear turquoise bags.
dull chartreuse bags contain 5 light tan bags.
bright beige bags contain 2 dim tomato bags.
mirrored yellow bags contain 5 drab aqua bags, 5 mirrored silver bags, 3 dark lime bags.
striped beige bags contain 4 dim white bags.
muted orange bags contain 3 drab beige bags, 5 faded olive bags.
drab turquoise bags contain 1 light green bag, 1 drab tomato bag, 4 clear purple bags.
plaid salmon bags contain 3 posh gray bags, 1 dim beige bag, 1 plaid brown bag.
faded gray bags contain 3 plaid fuchsia bags, 5 plaid magenta bags, 5 plaid white bags, 1 dull beige bag.
posh aqua bags contain 2 mirrored turquoise bags.
mirrored crimson bags contain no other bags.
vibrant magenta bags contain 1 dull purple bag, 4 shiny red bags, 5 drab chartreuse bags, 4 bright red bags.
striped yellow bags contain 1 shiny tan bag.
vibrant orange bags contain 4 plaid silver bags, 5 dim teal bags, 1 striped maroon bag, 3 plaid red bags.
dark indigo bags contain 3 drab purple bags.
shiny gray bags contain 1 wavy yellow bag.
faded plum bags contain no other bags.
dim blue bags contain 2 dotted crimson bags, 5 muted chartreuse bags, 3 dark silver bags, 4 vibrant lime bags.
muted white bags contain 4 muted teal bags, 3 posh purple bags, 1 dull lavender bag, 5 pale crimson bags.
muted lime bags contain 1 shiny turquoise bag, 2 faded indigo bags, 2 vibrant silver bags, 1 pale turquoise bag.
posh coral bags contain 5 faded violet bags.
dotted fuchsia bags contain 1 clear tan bag, 1 striped lavender bag.
wavy brown bags contain 2 dim aqua bags.
posh brown bags contain 1 shiny cyan bag, 5 plaid orange bags, 3 light coral bags.
drab purple bags contain 5 faded orange bags.
muted tan bags contain 2 wavy teal bags, 3 dim salmon bags, 1 bright gold bag, 3 clear gray bags.
dim yellow bags contain 1 vibrant gray bag, 4 vibrant lavender bags, 1 muted magenta bag.
pale salmon bags contain 2 vibrant beige bags, 3 bright maroon bags.
faded turquoise bags contain 4 plaid purple bags, 2 light tomato bags, 3 light salmon bags.
clear teal bags contain 4 muted yellow bags, 3 dim beige bags, 5 faded tomato bags, 1 dim red bag.
vibrant olive bags contain 3 muted lime bags, 5 shiny blue bags, 3 light gold bags, 3 dark olive bags.
dim beige bags contain 1 light cyan bag, 3 faded plum bags.
dull black bags contain 2 striped black bags, 5 wavy brown bags, 1 bright red bag, 1 drab teal bag.
pale turquoise bags contain 5 vibrant violet bags, 3 dotted fuchsia bags, 2 striped blue bags, 4 posh purple bags.
wavy red bags contain 3 drab teal bags.
faded gold bags contain 5 dull lavender bags.
faded black bags contain 5 wavy turquoise bags, 5 mirrored crimson bags.
bright orange bags contain 2 striped maroon bags, 4 light cyan bags, 5 light silver bags, 5 wavy indigo bags.
drab teal bags contain 4 wavy aqua bags, 5 light bronze bags, 1 drab red bag.
dull salmon bags contain 4 dull orange bags.
bright lavender bags contain 2 plaid silver bags, 3 bright crimson bags.
drab cyan bags contain 2 bright bronze bags, 5 bright crimson bags, 2 wavy yellow bags.
dull plum bags contain 3 shiny lavender bags, 2 bright magenta bags, 2 mirrored crimson bags, 4 mirrored silver bags.
dotted teal bags contain 3 muted yellow bags, 3 dotted turquoise bags, 5 mirrored white bags.
vibrant turquoise bags contain 5 clear magenta bags, 5 dark beige bags, 2 vibrant tan bags, 3 plaid blue bags.
pale crimson bags contain 2 bright turquoise bags, 4 dark lime bags.
pale plum bags contain 1 shiny chartreuse bag, 3 vibrant gold bags, 5 plaid violet bags, 5 dim salmon bags.
mirrored lime bags contain 2 plaid green bags.
pale lavender bags contain 5 dotted gold bags, 1 striped maroon bag, 3 shiny tan bags, 5 drab white bags.
muted olive bags contain 3 drab crimson bags, 2 dotted gray bags, 5 dotted tomato bags, 3 posh bronze bags.
drab brown bags contain 3 dull orange bags, 4 posh gold bags, 2 pale crimson bags, 1 plaid white bag.
bright tan bags contain 1 dark white bag, 5 bright turquoise bags.
faded indigo bags contain 3 pale yellow bags, 2 mirrored brown bags, 1 shiny gray bag, 5 bright turquoise bags.
dark aqua bags contain 1 posh gray bag, 4 striped coral bags, 2 posh olive bags, 2 bright white bags.
light tan bags contain 2 faded purple bags, 5 muted fuchsia bags.
dark magenta bags contain 4 posh silver bags, 5 pale plum bags, 5 muted violet bags, 2 faded green bags.
clear maroon bags contain 3 muted maroon bags, 1 clear olive bag, 2 faded black bags.
posh purple bags contain 4 faded orange bags, 4 shiny salmon bags.
bright gold bags contain 1 dull crimson bag, 1 shiny lavender bag.
dull purple bags contain 3 clear silver bags, 1 drab aqua bag, 3 wavy beige bags.
muted cyan bags contain 1 plaid green bag.
vibrant yellow bags contain 3 faded plum bags, 4 vibrant lavender bags, 2 muted magenta bags.
vibrant green bags contain 5 dim chartreuse bags, 2 posh lavender bags.
plaid bronze bags contain 4 dim tomato bags, 3 shiny silver bags.
striped blue bags contain 5 vibrant gold bags.
plaid coral bags contain 4 faded purple bags, 3 drab turquoise bags, 1 light teal bag, 2 posh indigo bags.
posh turquoise bags contain 5 posh cyan bags, 5 mirrored violet bags.
faded crimson bags contain 2 plaid salmon bags, 1 striped magenta bag, 1 striped maroon bag.
striped purple bags contain 1 drab violet bag, 2 pale maroon bags, 2 drab beige bags, 4 faded maroon bags.
clear orange bags contain 5 dotted red bags, 4 mirrored red bags.
striped fuchsia bags contain 2 bright tomato bags, 4 dull magenta bags.
wavy purple bags contain 5 wavy olive bags, 3 shiny silver bags.
dotted crimson bags contain 1 striped teal bag, 5 shiny tomato bags.
dull lavender bags contain 1 dotted tomato bag, 3 wavy indigo bags, 1 faded silver bag.
dotted magenta bags contain 3 dotted coral bags.
drab silver bags contain 5 dark indigo bags, 4 bright bronze bags, 1 dim lime bag, 3 bright aqua bags.
clear cyan bags contain 1 bright maroon bag.
shiny beige bags contain 2 plaid beige bags, 2 striped magenta bags, 4 drab yellow bags, 4 muted yellow bags.
dim turquoise bags contain 2 dull tan bags.
drab coral bags contain 4 striped bronze bags, 3 dull orange bags, 5 plaid silver bags.
light lavender bags contain 1 wavy plum bag, 4 posh purple bags.
posh yellow bags contain 3 dark lime bags, 3 light cyan bags, 4 dull crimson bags.
posh violet bags contain 2 faded chartreuse bags.
shiny teal bags contain 5 mirrored salmon bags, 1 wavy plum bag, 3 posh purple bags, 2 posh cyan bags.
vibrant lime bags contain 1 dotted tomato bag, 1 drab fuchsia bag, 1 mirrored brown bag.
dark fuchsia bags contain 1 vibrant blue bag.
bright green bags contain 4 dark purple bags, 4 bright tan bags, 5 drab turquoise bags.
dull coral bags contain 2 dotted tomato bags.
posh white bags contain 3 dull bronze bags, 1 faded salmon bag, 3 pale tomato bags, 3 vibrant salmon bags.
plaid black bags contain 4 vibrant aqua bags, 2 shiny teal bags.
posh lavender bags contain 4 wavy beige bags, 1 dim silver bag, 2 faded white bags.
posh orange bags contain 4 dim orange bags, 3 faded chartreuse bags.
plaid brown bags contain 5 shiny fuchsia bags, 5 striped magenta bags, 2 wavy coral bags, 1 vibrant beige bag.
shiny plum bags contain 3 light green bags, 4 bright orange bags.
dim aqua bags contain 4 faded gold bags, 1 dotted lavender bag, 3 light gold bags, 4 shiny aqua bags.
wavy gold bags contain 4 clear violet bags, 5 dim red bags, 5 drab crimson bags, 2 dull coral bags.
vibrant tomato bags contain 3 faded olive bags.
bright silver bags contain 2 dull lavender bags, 4 faded aqua bags, 4 dim coral bags.
wavy turquoise bags contain 4 dull lavender bags, 1 drab aqua bag.
wavy aqua bags contain 3 wavy yellow bags.
light indigo bags contain 5 dim beige bags, 3 drab fuchsia bags, 1 plaid violet bag, 5 bright yellow bags.
striped red bags contain 3 bright violet bags, 5 dark indigo bags, 5 dim silver bags.
posh lime bags contain 5 striped teal bags, 3 dark brown bags, 4 bright crimson bags, 2 posh violet bags.
faded magenta bags contain 5 light gold bags, 1 posh indigo bag, 3 plaid lime bags.
vibrant lavender bags contain 5 faded maroon bags, 3 striped orange bags, 3 dull crimson bags.
clear purple bags contain 5 dark green bags, 3 light cyan bags, 3 clear white bags, 4 pale silver bags.
muted purple bags contain 5 drab fuchsia bags, 1 vibrant salmon bag, 5 bright yellow bags.
bright lime bags contain 5 vibrant violet bags, 5 wavy maroon bags.
bright black bags contain 5 dark silver bags, 2 vibrant green bags.
muted magenta bags contain no other bags.
striped tomato bags contain 4 shiny gold bags.
pale black bags contain 3 dull brown bags, 2 wavy coral bags, 2 posh purple bags, 4 faded beige bags.
dim red bags contain 1 dotted tomato bag, 2 wavy beige bags.
light black bags contain 4 dull maroon bags, 3 dotted aqua bags, 3 drab purple bags.
shiny turquoise bags contain 3 dull blue bags, 4 posh bronze bags.
pale indigo bags contain 4 dotted tomato bags.
shiny white bags contain 5 vibrant tomato bags, 4 dotted magenta bags, 3 dull aqua bags.
dark lavender bags contain 4 faded orange bags.
shiny brown bags contain 3 bright orange bags, 2 dotted crimson bags, 2 wavy aqua bags.
light chartreuse bags contain 1 muted lavender bag.
faded aqua bags contain 4 dull green bags, 4 faded violet bags, 4 clear gray bags.
light olive bags contain 4 shiny purple bags, 3 shiny plum bags.
wavy indigo bags contain 3 dim green bags, 5 shiny lavender bags, 3 posh olive bags, 1 dull crimson bag.
dull tomato bags contain 3 light coral bags, 2 light cyan bags, 3 plaid silver bags.
clear turquoise bags contain 1 wavy crimson bag, 4 dim tomato bags.
dim tomato bags contain 5 shiny lavender bags, 2 dim gold bags.
wavy black bags contain 4 wavy chartreuse bags, 4 dull coral bags.
dotted maroon bags contain 5 pale violet bags, 5 vibrant aqua bags, 5 plaid cyan bags.
clear lavender bags contain 2 dotted turquoise bags, 4 light purple bags, 1 plaid plum bag.
dotted cyan bags contain 2 dotted turquoise bags, 4 plaid gold bags, 5 drab red bags, 5 faded orange bags.
muted violet bags contain 3 light teal bags, 2 striped gold bags.
wavy silver bags contain 2 dim coral bags, 1 shiny chartreuse bag, 4 shiny turquoise bags.
drab crimson bags contain 2 drab beige bags.
dull aqua bags contain 4 striped maroon bags, 5 vibrant beige bags, 3 faded gold bags, 3 dark lime bags.
vibrant coral bags contain 4 faded gold bags, 2 dotted black bags, 5 drab tomato bags.
pale beige bags contain 1 dotted coral bag.
dim olive bags contain 5 clear gray bags, 2 muted violet bags, 3 clear gold bags, 4 shiny red bags.
dim cyan bags contain 4 plaid silver bags.
muted bronze bags contain 4 wavy yellow bags, 1 vibrant yellow bag, 5 dim tomato bags, 2 mirrored silver bags.
dim gold bags contain 2 dim green bags, 4 shiny tomato bags, 4 striped white bags, 4 mirrored crimson bags.
vibrant gray bags contain 4 dotted turquoise bags, 4 posh violet bags.
plaid orange bags contain 2 dotted turquoise bags.
posh magenta bags contain 4 dotted lime bags.
light silver bags contain 5 shiny gold bags, 3 dull tan bags.
bright plum bags contain 5 pale magenta bags, 5 dull aqua bags.
drab red bags contain 1 dim gold bag.
striped salmon bags contain 1 vibrant lavender bag, 3 wavy crimson bags, 5 posh olive bags.
dotted purple bags contain 3 dull lavender bags.
bright violet bags contain 2 dotted indigo bags, 4 shiny violet bags.
muted red bags contain 2 mirrored silver bags.
dotted green bags contain 5 muted plum bags.
clear lime bags contain no other bags.
wavy gray bags contain 1 posh cyan bag, 5 dim bronze bags, 5 posh lime bags, 4 drab silver bags.
dotted brown bags contain 4 striped tomato bags.
plaid indigo bags contain 1 plaid purple bag, 3 dark turquoise bags, 3 light purple bags, 5 dotted cyan bags.
mirrored olive bags contain 5 bright gold bags, 3 dotted gold bags, 4 drab gray bags, 4 drab tomato bags.
wavy green bags contain 3 mirrored indigo bags, 4 wavy chartreuse bags, 2 clear cyan bags, 5 bright violet bags.
light gray bags contain 4 vibrant yellow bags, 2 dull blue bags.
posh plum bags contain 3 wavy yellow bags, 5 dim yellow bags, 2 clear chartreuse bags.
posh bronze bags contain 2 clear lime bags.
bright indigo bags contain 4 muted salmon bags, 2 posh violet bags.
mirrored violet bags contain 1 faded plum bag, 3 dull orange bags.
muted silver bags contain 3 mirrored indigo bags, 4 shiny fuchsia bags.
striped coral bags contain 2 posh turquoise bags.
plaid tomato bags contain 1 wavy beige bag, 5 muted green bags.
dark gold bags contain 1 clear gold bag, 2 light maroon bags, 3 pale teal bags.
light green bags contain 1 dull aqua bag, 4 dotted magenta bags, 4 dull purple bags.
plaid lime bags contain 1 faded fuchsia bag.
muted beige bags contain 1 dotted coral bag, 1 plaid fuchsia bag, 2 posh bronze bags.
pale red bags contain 2 wavy gold bags, 1 striped lavender bag.
striped lime bags contain 1 wavy turquoise bag, 4 dim aqua bags.
muted black bags contain 3 dotted black bags.
faded brown bags contain 1 dim turquoise bag, 2 plaid brown bags.
plaid blue bags contain 2 muted salmon bags.
muted indigo bags contain 2 dull fuchsia bags, 2 dull lime bags.
dark coral bags contain 2 faded indigo bags, 3 shiny tomato bags, 2 mirrored salmon bags.
vibrant beige bags contain 4 vibrant lavender bags.
mirrored bronze bags contain 1 mirrored orange bag, 5 plaid red bags, 1 mirrored cyan bag.
shiny red bags contain 1 bright yellow bag.
light crimson bags contain 5 plaid crimson bags, 3 pale white bags, 3 drab fuchsia bags, 3 muted olive bags.
drab blue bags contain 1 mirrored black bag, 3 striped fuchsia bags, 3 drab tan bags.
striped violet bags contain 2 muted gray bags, 5 faded white bags, 5 striped black bags, 3 mirrored tomato bags.
wavy maroon bags contain 2 light cyan bags, 3 muted teal bags, 2 vibrant lavender bags.
pale brown bags contain 2 wavy black bags.
bright aqua bags contain 3 plaid tan bags.
muted plum bags contain 5 light bronze bags, 1 dotted beige bag, 3 vibrant cyan bags.
posh green bags contain 4 faded lime bags, 1 dull coral bag, 4 mirrored silver bags.
dark beige bags contain 3 wavy yellow bags, 5 dim salmon bags.
light fuchsia bags contain 4 plaid coral bags.
shiny tan bags contain 2 drab tan bags, 4 clear gray bags, 2 mirrored crimson bags, 1 vibrant beige bag.
clear crimson bags contain 3 vibrant aqua bags, 4 dotted indigo bags.
clear beige bags contain 4 posh olive bags, 4 bright lime bags, 3 dark green bags.
dim gray bags contain 1 drab beige bag, 3 clear tomato bags, 2 mirrored fuchsia bags.
light violet bags contain 3 plaid purple bags.
faded silver bags contain 5 striped bronze bags, 4 dotted lavender bags.
dotted white bags contain 5 drab green bags.
shiny purple bags contain 3 posh turquoise bags, 2 dark brown bags.
posh blue bags contain 2 vibrant chartreuse bags, 2 muted olive bags, 4 dull brown bags, 5 wavy magenta bags.
striped cyan bags contain 1 pale plum bag, 3 dim maroon bags, 5 light salmon bags, 3 dim crimson bags.
shiny tomato bags contain no other bags.
plaid aqua bags contain 4 clear maroon bags, 4 plaid white bags, 2 posh chartreuse bags.
pale magenta bags contain 5 muted teal bags, 1 vibrant lavender bag.
dim plum bags contain 4 drab coral bags.
drab beige bags contain 4 wavy crimson bags, 1 dull tan bag, 3 dotted tomato bags.
wavy coral bags contain 1 mirrored yellow bag, 1 dull tan bag, 5 drab gold bags, 2 muted bronze bags.
drab violet bags contain 4 faded maroon bags, 2 posh gold bags, 3 wavy maroon bags, 3 bright lime bags.
light aqua bags contain 5 light bronze bags, 1 light tan bag, 1 dull beige bag.
light orange bags contain 1 dark tan bag, 3 dim white bags, 3 plaid plum bags.
pale chartreuse bags contain 3 pale violet bags, 1 drab fuchsia bag, 1 shiny indigo bag, 2 dull turquoise bags.
faded chartreuse bags contain 4 vibrant lime bags, 3 drab tomato bags, 1 vibrant violet bag, 1 vibrant tomato bag.
clear chartreuse bags contain 2 dotted plum bags, 1 muted brown bag, 2 wavy chartreuse bags, 4 faded green bags.
dotted silver bags contain 3 mirrored yellow bags.
dotted violet bags contain 4 muted turquoise bags, 4 light black bags.
wavy orange bags contain 3 shiny fuchsia bags, 4 clear beige bags.
dark red bags contain 1 faded salmon bag, 3 muted purple bags.
dim bronze bags contain 2 clear turquoise bags.
light turquoise bags contain 5 dark red bags.
muted green bags contain 5 bright turquoise bags, 2 wavy coral bags, 3 faded chartreuse bags.
vibrant violet bags contain 3 wavy indigo bags, 1 dotted gray bag, 4 vibrant beige bags.
dull cyan bags contain 3 light maroon bags, 2 posh plum bags.
muted aqua bags contain 2 dotted beige bags, 2 faded yellow bags, 3 plaid gray bags, 3 bright chartreuse bags.
dark tomato bags contain 2 striped purple bags, 2 dark maroon bags, 2 dim silver bags.
vibrant blue bags contain 4 dull orange bags.
faded violet bags contain 2 vibrant lavender bags, 5 plaid plum bags, 4 bright magenta bags, 4 faded silver bags.
bright salmon bags contain 1 dull crimson bag, 2 light maroon bags.
vibrant aqua bags contain 4 dotted aqua bags, 3 vibrant gray bags, 3 dotted lavender bags.
striped plum bags contain 2 mirrored lime bags, 2 dark salmon bags.
plaid gold bags contain 4 dark lime bags, 3 drab aqua bags, 3 dim white bags, 2 mirrored brown bags.
"""
