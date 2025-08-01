The quote mixer is a simple CLU born out of a game me and some friends like to play on Discord.

It goes a little something like this:  
    1. Take a message someone else said.  
    2. Through a combination of removing parts of the original message and adding new text, change the message into something very different.  
    3. Added text must have '`' characters wrapped around it. In Discord this formats the text to be smaller and in a dark box.  
    4. Removing parts of the original message is preferable to adding new text. The addition of new text has a sort of "cost" and a lot of the comedy comes from how dramatically you change a message through very simple additions.  
    
This program takes in two strings and an insertion seperator ('`' if none is used) as arguments and produces the most optimal transformation from the first string to the second. This is done by finding the longest common subsequence between the two strings and making insertions as needed.  

Example:  
    Here is how to mix the first paragraph of War and Peace into Eddie the Yeti's song from the Donkey Kong Country animated series.  
    `No! I can't b`elie`ve` `what I `s`ee` `Ev`e`rythi`n`g's upside d`o`wn,` and `it` `m`a`k`e`s` no s`ense `t`o` m`e In m`y `curr`e`nt `state`,` `I` `can'`t` c`h`ang`e t`h`e`ir` `fa`t`e` If you `se`n`d` `m`e` back, I swear I'`ll `s`e`t` th`ings str`a`igh`t `You go`t`ta `s`end` me` b`a`ck!` `No c`a`n` `do! I've been d`i`ssed!` `Y`ou `not exi`st`! Edd`i`e,` `le`t `me go back `to `my hom`e `Wi`th`out m`e`,` `everyth`in`g'`s a`ll` `w`ro`ng` `Eddi`e`, l`et` m`e `go `b`ack` t`o my `h`ome Le`t `me pu`t` t`hi`ng`s `back whe`re` the`y bel`ong Edd`ie`, l`e`t` `m`e `go` `back `t`o my home W`i`t`h`out me, eve`r`yth`i`ng'`s `all `w`rong Edd`i`e, `l`et` `me go back to my `h`ome Ple`a`s`e`!` `I'm dow`n` `on m`y knees The`re`'s` `a` `w`o`rld` th`at` `needs` `s`a`vi`n`g` `Bab`y` B`o`bby Eddie Yeti, j`u`st` `l`e`t` `me be Oh ma`n`!` `W`on`'t you h`e`a`r my `pl`e`a? Come o`n, `c`o`me` on`, Eddi`e`,` y`ou` `go`t` to `h`e`l`p` `m`e `You gott`a` `s`end` `me` `b`a`ck!` `Me n`o`t s`ure! `J`u`s`t o`ne` d`ay? N`o `wa`y`,` `J`o`sé!` `Eddie,` `l`e`t m`e `go` `b`a`ck to my home Without me, e`ve`rything's` `all w`r`ong Edd`i`e, let me `g`o back to my `h`ome Le`t` m`e` put thi`n`gs back wh`e`re` `the`y` bel`o`ng Edd`i`e, le`t `me g`o `b`a`ck` t`o my hom`e `Without `me`,` `everything's `all `wrong Eddie, le`t` m`e `go back to my hom`e.  

TODO:  
    1. Have the ability to return *all* possible transformations; two strings can have several different LCSs.  
    2. Reorganize file structure to look like a proper Rust project (ie: not having everything just in main.rs). This would also make it more viable to use the quote mixer as a library.  
    3. Add the ability to use files at inputs.  
    4. Optimize the program to remove similar beginnings and endings (ie: "AAABBBCCC" and "AAADDDCCC" only actually operates on "BBB" and "DDD").  
    5. Having this usable as an actual Discord bot would be pretty neat but I do not know how to do that and it would probably be unnecessary (see below), but if you want to go ahead.  

Credits:  
    Code and what-not: Me  
    The thing that actually makes this possible: Whoever wrote the Wikipedia page on how to find the LCS of two strings  

Disclaimer:  
    This program was not written to optimize anyones ability to do the bit. Typically what a message is turned into is born from what the author recognizes as simple changes. You usually don't go into it with an output string in mind. Either way using a program like this would kinda take the magic away. It's still really funny to see what happens to large inputs a human could never possibly do.  
