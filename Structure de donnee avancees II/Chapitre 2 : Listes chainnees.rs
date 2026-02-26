INTRODUCTION 
Une liste chainee est une structure de donnees lineaire utilisee en programmation pour stocker
une collection d'elements.
  Contrairement aux tableaux qui sont stockees en memoire de maniere contigue.

Une liste chainnees est compossee d'une liste de blocs nommes "noeuds" , chacun contenant : 
- Une valeur (donnee) ; 
- Un lien (pointeur) vers le noeud suivant ;
- 

Une liste chainee est une structure lineaire qui n'as de dimension fixee a sa creation, ses elements
de meme type sont eparpilles dans la memoire et relies entre eux par des pointeurs. Sa dimension (taille)
peut etre modifiee selon la place disponible en memoire. La liste est accessible uniquement par sa tete de
liste (Premier Element)

 I - Interet des listes chainees : 
On utilise les listes chainnees lorsque l'on doit traiter des onjets representes par suites sur lesquelles
on doit effectuer de nombreuse suppression et de nombreux ajouts

 1 - Avantages : 
 - Taille dynamique / flexible : la taille de liste peut varier a l'execution , pas besoin de savoir d'avance
 la taille de la structure ; 
 - Insertion / Suppression efficace : ces operations sont rapidement traitees dans la dite structure de donnee
 car on a pas besoin de deplacer les elements contrairement aux tableaux
 - Optimisation de la Memoire : pas besoin d'allouer/assigner/attribuer un bloc contigu de memoire

 2 - Inconvenients : 
 - Access sequentiel uniquement : Impossible d'acceder directement a un element par son indice comme les tableaux
 car il faut parcourir la liste 
 - Surcout de la memoire : chaque noeuds necessite de l'espace supplementaire pour stocker le pointeur
 
 II - Les types de listes chainees : 

 Il existe differents types de listes chainees , a savoir : 
 - Listes chainees simplement : constituee d'elements relies entre eux par des pointeurs
 - Listes doublement chainees : avec un parcours dans le sdeux sens 
 - Listes circulaires

    Ces differents types peuvent etre mixes selon les besoins.
     
    1 - Listes simplement chainees : 
    voir(liste-chainee.png)

    - Chaque cellule (noeud) contient en plus des donnees , un pointeur vers l'element suivant de la liste, le pointeur
    du dernier element a pour valeur NULL 
    -  Une variable appelee "Premier" (tete) contient l'adresse du premier element de la liste 
    - Lorsque la liste est vide , la tete contient la valeur N    - Chaque cellule (noeud) contient en plus des donnees , un pointeur vers l'element suivant de la liste, le pointeur
    du dernier element a pour valeur NULL 
    -  Une variable appelee "Premier" (tete) contient l'adresse du premier element de la liste 
    - Lorsque la liste est vide , la tete contient la valeur NULL ULL 
   
    a - Operation susceptible d'etre manipulee sur une telle liste : 

     * Ajout d'un element en tete de liste , son principe est : 
      -> Etape 1 : Commencer par creer une nouvelle cellule et initialiser son champs "info"
      -> Etape 2 : Ajouter le nouvel element en debut de la liste en le faisant pointer vers le second element de la liste
      -> Etape 3 : Mettre a jour la liste ie. le premier element ne contient plus l'adresse de debut qui est maintenant dans nouveau.
      
    //Algorithme

    structure Noeud
        valeur
        suivant
    finstructure

    Algorithme Insertionentete(Liste,X)
        Nouveau <--- allouer Noeud; //Cree un nouveau noeud en memoire
        Nouveau.valeur <--- X; //Stocker la valeur X dans le nouveau noeud
        Nouveau.suivant <--- Liste.tete; //Relie le nouveau noeud a l'ancien premier noeud
        Liste.tete <--- Nouveau; //Mettre a jour, pour que la tete de liste pointe vers le nouveau noeud
    FinAlgorithme

     * Ajout d'un element en fin de liste , son principe est : 
      -> Etape 1 : Commencer par creer un nouveau noeud et initialiser son champs "info"
      -> Etape 2 : Deux cas de figures peuvent se presenter(liste vide ou non):  
        --> Si la liste est vide alors pointer le suivant du noeud a inserer vers NULL sinon ce positionner sur le dernier element de
        la liste et ajouter le nouveau noeud a la suite.
          l'algorithme est le suivant  :

          //Algorithme 
          Algorithme InsertionEnFin(Liste, X) 
            Nouveau <--- AllouerNoeud; //Creer un nouveau noeud en memoire
            Nouveau.valeur <--- X; //Stocke la valeur X
            Nouveau.suivant <---NULL; //Initialiser suivant a NULL (fin de liste)
                Si (Liste.tete = NULL) Alors //Si liste vide,
                    Liste.tete <--- Nouveau; //Nouveau devient la tete
                Sinon
                    Courant <--- Liste.tete;
                    Tantque (Courant.suivant != NULL) Faire //Parcourt la liste jusqu'au dernier element
                        Courant <--- Courant.suivant;
                    Fintantque
                Courant.suivant <--- Nouveau; //Relie le dernier noeud au nouveau noeud
                Finsi
          FinAlgorithme
     * Suppression d'un element dans une liste simplement chainee , son principe est :
     


