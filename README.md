NB: has to be build as follows: 

`cargo +stable-i686-pc-windows-msvc run`

The DLLs (originally from [here](http://www.eki.ee/tarkvara/analyys/ana_bin.zip)) have to be brought into the folder with executable file

// TODO: add build phase to deploy DLLs alongside the executable 
 
Environment variable `EST_MORPHO_DATA` has to be set to a folder, 
containing "data" files (one should download them from [here](http://www.eki.ee/tarkvara/est_morpho_data.zip))

## Usage example:

`rust-est.exe tulla`:

```
Analyze for "tulla":
tulla =tull  (Inf  >tulema !36_V
tulla =tulla  (SgN  >tulla !16_S
tulla =tulla  (SgG  >tulla !16_S
tulla =tulla  (IndPrPs_  >tullama !29_V
tulla =tulla  (ImpPrSg2  >tullama !29_V
```

## Decyphering output:

Cf. below

Based on the output above, we can tell that `tulla` can be either:
- an _infinitive_ form of a verb `tulema` that has conjugation type 36
- a _nominative singular_ or a _genitive singular_ of a noun `tulla` that has declension type 16
- an _imperative mood_ or a _negative present form_ of a verb `tullama` that has conjugation type 
 
### Parts of speech  

(taken from [here](http://www.eki.ee/keeletase/#/))

| Short | Explained | English | Russian |
| ----------- | ----------- | ----------- | ----------- |
| A | omadussõna | adjective | прилагательное | 
| D | määrsõna | adverb | наречие | 
| G | käändumatu omadussõna | un-inclinable adjective | несклоняемое прилагательное |
| I | hüüdsõna | interjection | междометие | 
| J | sidesõna | conjunction | союз |
| K | kaassõna | preposition | предлог |
| N | põhiarvsõna | cardinal | количественное числительное |
| O | järgarvsõna | ordinal | порядковое числительное |
| P | asesõna | pronoun | местоимение |
| S | nimisõna | noun | существительное |
| V | tegusõna | verb | глагол |
| Y | lühend | abbreviation | сокращение |
| X | verbi juurde kuuluv sõna, millel eraldi sõnaliigi tähistus puudub, nt plehku| - | - |
 
### Other abbreviations: 
 
(taken from [here](http://www.eki.ee/tarkvara/morf_lisa.html))
 
 väärtus = lühend	nimetus
 
VERB:
1. infiniitsed (e käändelised) vormid
1 = Inf	infinitiiv e da-infinitiiv e da-tegevusnimi
2 = Ger	gerundium e des-vorm
3 = Sup	supiin e ma-infinitiiv e ma-tegevusnimi
4 = Pts	partitsiip e kesksõna
2. tegumood
0 = Ps	personaal e isikuline tegumood
1 = Ips	impersonaal e umbisikuline tegumood
3. aeg
0 = Pr	preesens e olevik
1 = Pt	preteeritum e (üld)minevik
2 = Ipf	imperfekt e lihtminevik
3 = Pf	perfekt e täisminevik
4 = Ppf	pluskvamperfekt e enneminevik
4. kõneviis
0 = Ind	indikatiiv e kindel kõneviis
1 = Kvt	kvotatiiv e kaudne kõneviis
2 = Knd	konditsionaal e tingiv kõneviis
3 = Imp	imperatiiv e käskiv kõneviis
5. arv-isik
0 = 1-6	arvu-isikut ei eristata (kõik pöörded)
1 = Sg 1	singulari e ainsuse 1. pööre
2 = Sg 2	singulari e ainsuse 2. pööre
3 = Sg 3	singulari e ainsuse 3. pööre
4 = Pl 1	pluurali e mitmuse 1. pööre
5 = Pl 2	pluurali e mitmuse 2. pööre
6 = Pl 3	pluurali e mitmuse 3. pööre
6. kõnelaad
0 = 1-2	kõnelaadi ei eristata (jaatav või eitav kõne)
1 = Af	afirmatiiv e jaatav kõne
2 = Neg	negatiiv e eitav kõne
 
NOOMEN:
7. arv
0 = Sg	singular e ainsus
1 = Pl	pluural e mitmus
8. kääne
N = Nom	nominatiiv e nimetav
G = Gen	genitiiv e omastav
P = Part	partitiiv e osastav
D = Adt	aditiiv e suunduv (e lühike sisseütlev)
1 = Ill	illatiiv e sisseütlev
2 = In	insessiiv e seesütlev
3 = El	elatiiv e seestütlev
4 = All	allatiiv e alaleütlev
5 = Ad	adessiiv e alalütlev
6 = Abl	ablatiiv e alaltütlev
T = Tr	tranlatiiv e saav
R = Ter	terminatiiv e rajav
E = Es	essiiv e olev
A = Ab	abessiiv e ilmaütlev
K = Kom	komitatiiv e kaasaütlev