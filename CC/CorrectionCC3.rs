Nombre premier : un nombre premier est un nombre qui a deux diviseurs distincts : 1 et lui-même. Par exemple, 2, 3, 5, 7, 11 sont des nombres premiers.

Exercice 5 :

1- Inserons les valeurs :

h(12) = 12 mod 10 = 2
h(22) = 22 mod 10 = 2
h(33) = 33 mod 10 = 3
h(7) = 7 mod 10 = 7
h(17) = 17 mod 10 = 7
h(27) = 27 mod 10 = 7

               *Representation dans un tableau :

   --------------------------------------------------------------
   |Valeur | h(x) = x mod 10 | case | Action                    |
   --------------------------------------------------------------
   |  12   |  12 mod 10      |  2   | inserer 12                |
   --------------------------------------------------------------
   |  22   |  22 mod 10      |  2   | colision: ajouter apres 12|
   --------------------------------------------------------------
   |  33   |  33 mod 10      |  3   | inserer 33                |
   --------------------------------------------------------------
   |  7    |   7 mod 10      |  7   | inserer 7                 |
   --------------------------------------------------------------
   |  17   |  17 mod 10      |  7   | colision: ajouter apres 7 |
   --------------------------------------------------------------
   |  27   |  27 mod 10      |  7   | colision: ajouter apres 17|
    -------------------------------------------------------------


    3- Calcul del h(27) :
    bla bla bla (comparaisons entre les differentes cles aux indices)


Exercice 2 :

1 -

*1ere methode :
 Fonction Nbpremier(N: entier):entier
 var i : entier
 Debut
    Ecrire("Entrez un nombre non nul et different de 1 ");
    lire(N);
    Pour (i <- 2 a N-1) faire
        si (N%i = 0) alors
            retourner 0;
        finsi
    Finpour
 Fin

 *2eme methode :

Fonction Nbpremier(N: entier):entier
 var i,c : entier;
 Debut
    c <- 0;
    i <- 1;
    Lire(N);
    Tantque (N>= i) faire
        si (N%i = 0) alors
            c <- c + 1;
        finsi
    i <- i+1;
    Fintantque
    Si (c = 2) alors
        retourner 1;
    sinon
        retourner 0;
    finsi
FinFonction


2- 
U0=1
Un+1 = 5un + 3

Fonction suite(n: entier):entier
    si(n = 0) alors
        retourner 1;
    sinon
        retourner 5*suite(n-1) + 3;
    finsi
FinFonction

Exercice 3 :

Algorithme MHS
var Pu,MHS: reel;
NH : entier;

Debut
    Ecrire("Entrer le nombre d'heures de travails");
    lire(NH);
    Ecrire("Entrer le prix unitaire de l'heure");
    lire(pu);
    si (NH <= 39) alors
        MHS <- 0;
    sinon


Exercice 6 :

fo = f1 = 1
pour toute valeurs de n > 1, Fn = Fn-2 + Fn-1;

Algorithme suitefibo
var Fo,F1,Fn, i: entier;
Debut
    Fo <-1 ; F1 <- 1:
    Ecrire("Fo =",Fo);
    Ecrire("F1 =",F1);
    Pour (i <-2 a 9) faire
        Fn <- Fo + F1;
        Ecrire("Fn",i,"=",Fn);
        Fo <- F1;
        F1 <- Fn;
    Finpour
Fin

Exercice 7 :

    Algorithme Notes
    var n,i: entier
    var note,min,max,som,M: reels
    Debut
        Ecrire("Entres le nombr de notes");
        lire(n);
        som <- 0, min <- 0, max <- 0;
        Pour (i<- 1 a n) faire
            Ecrire("Entrer une note");
            lire(note);
        Finpour
        Tantque (note >= 0 et note <= 20) faire
            som <- som + note;
            si (note < min) alors
                min <- note;
            finsi
            si(note > max) alors
                max <- note;
            finsi
        Fintantque
        M <- som/n;
        Ecrire("La meilleure note est: ",max);
        Ecrire("La mauvaise note est: ",min);
        Ecrire("La moyenne de toutes les notes est: ",M);
    Fin





