Complexite Temporelle (Temps d'execution)
Complexite Spatiale (Espace Memoire)
 ** son avantage est de comaparer et chosirir les algorithmes les plus efficaces/ les plus rapides pour resoudre un probleme donne.
 en fonction de l'environnement , de l'espace , des evenements

I - Introduction :

La Complexite Algorithmique est un domaine essentiel en informatique qui se concentre sur l'efficacite des algorithmes et des programmes.
> Elle Etudie la quantite de ressources (temps (complexite temporelle) et espace(complexite spatiale)) necessaires pour executer un algorithme en fonction de la taille de l'entree.
> Elle est essentielle pour comparer et choisir les algorithmes les plus efficaces pour resoudre un probleme donne.

II -Importance de la complexite : 
 
 Son analyse est essentielle pour plusieurs raisons :
    
    1-  Comparaison des algorithmes : 
         Elle permet de comparer l'efficacite de differents algorithmes pour resoudre un 
         meme probleme et de choisir le plus approprie en fonction des "Contraintes de Temps et d'Espace".
         
    2- Optimisation des performances :
         En comprenant la "complexite d'un algorithme", les developpeurs peuvent l'optimiser pour qu'il fonctionne plus efficacement sur de
         grandes quantites de donnees.

    3- La Prediction / Anticipation des performances :
         Elle permet de predire comment un algorithme se comportera avec des "entrees de tailles variables"(lorsque la taille des donnees en entree augmentera),
          ce qui est crucial pour la planification des ressources et permettant ainsi de concevoir des systemes informatiques viables et performants.

III - Analyse de la complexite temporelle :
    
    1 - Notation de complexite :
    Dans le paradigme de la complexite  plusieurs notations sont couramment utilisees dont les plus importantes sont :


   **Notations asymptotiques :

  Les notations asymptotiques sont des outils mathematiques utilises pour decrire le comportement d'un algorithme en fonction de la "taille de l'entree."
  Elles permettent d'exprimer la complexite temporelle et spatiale d'un algorithme de maniere concise et standardisee.
  
    a- Notation Grand O (Big O) :
        Elle represente la "limite superieure" de du temps d'execution d'un algorithme dans "le pire des cas". [1 ------ >n<]
           Par exemple : un algoritme dont le temps d'execution est O(N^2) signifie que le temps d'execution augmente proportionnellement au carre (Quadratiquement) de la taille de l'entree (N).
               Suggest A:
                // La notation Big O est utilisee pour decrire la limite superieure de la complexite d'un algorithme.
                // Elle indique le pire des cas en termes de temps d'execution ou d'espace memoire requis en fonction de la taille de l'entree (n).
                // Par exemple, si un algorithme a une complexite de O(n^2), cela signifie que le temps d'execution augmente proportionnellement au carre de la taille de l'entree.

    b- Notation Omega (Ω) :
        La notation Omega est utilisee pour decrire la "limite inferieure" du temps d'execution de la l'algorithme dans "le meilleur des cas". [>1< ------ n]
                // Elle indique le meilleur des cas en termes de temps d'execution ou d'espace memoire requis en fonction de la taille de l'entree (n).
                // Par exemple, si un algorithme a une complexite de Ω(n), cela signifie que le temps d'execution est au moins proportionnel a la taille de l'entree.

    c- Notation Theta (Θ) :
        La notation Theta est utilisee pour decrire une "borne serrée" sur le temps d'execution indiquant a la fois la "limite inferieure" et la "limite superieure" d'un algorithme.

         Exemple :
           - L'access direct a un element dans un tableau est Θ(1) car il prend un temps constant, peu importe la taille du tableau. (COMPLEXITE CONSTANTE)
           - Le "parcours d'un tableau de n elements" pour trouver un element specifique est Θ(n) car "dans le pire des cas, chaque element doit etre verifie une fois". (COMPLEXITE LINEAIRE)
           - La "double boucle imbriquee" parcourant un tableau de n elements a une complexite de Θ(n^2) car "chaque element est compare avec chaque autre element". (COMPLEXITE QUADRATIQUE)
                Suggest C:
                // La notation Theta est utilisee pour decrire une borne serrée sur la complexite d'un algorithme.
                // Elle indique que le temps d'execution ou l'espace memoire requis est proportionnel a une certaine fonction en fonction de la taille de l'entree (n).
                // Par exemple, si un algorithme a une complexite de Θ(n log n), cela signifie que le temps d'execution est proportionnel a n log n dans les meilleurs et pires cas.
    
    2- Classes de complexite courantes :

    Plusieurs classes sont utilisees dans le domaine de la complexite et les plus courantes sont :

        a- Complexite constante O(1) :
            Le temps d'execution de cet algorithme est et reste constant, "peu importe la taille de l'entree" si le temps d'acces a chacun des elements est similaire dans l'ensemble etudie.
            Exemple : 
                > Acces direct a un element dans un tableau,
                > Insertion ou suppression d'un element dans une table de hachage,
                > Verification si une pile est vide,
                > Verification si une file est pleine,
                > Recuperation de la taille d'une liste chainee (si la taille est stockee explicitement)
        
        b- Complexite logarithmique O(log n) :
            Le temps d'execution augmente proportionnellement au logarithme (logarithmiquement avec la taille de l'entree) de la taille de l'entree. POur cela les algorithmes de ce type sont
            souvent tres efficaces pour traiter de grandes quantites de donnees. La Complexite logarithmique signifie quele temps d'execution de l'algorithme augmente de maniere logarithmique
            par rapport a la taille de l'entree.
            Exemple : 
                > Recherche binaire dans un tableau trie (permet de reduire le temps de recherche).
                > Recherche par tri par fusion (diviser la liste en plusieurs parties).
                > Recherche dans des structures de donnees arborescentes (arbres binaires de recherche).
                > Recherche dans des graphes avec des algorithmes optimises.
                > Recherche dans des bases de donnees indexees.

        c- Complexite lineaire O(n) :
            Le temps d'execution augmente lineairement avec la taille de l'entree. Autrement dit, si la taille de l'entree double, le temps d'execution double egalement.
            Exemple : 
                > Parcours d'un tableau pour trouver un element specifique.
                > Somme des elements d'une liste.
                > Recherche d'un maximum ou minimum dans un tableau non trie.
                > Copie des elements d'un tableau a un autre.
                > Calcul du factoriel d'un nombre n (n!).

        d- Complexite quadratique O(n^2) :
            Le temps d'execution augmente proportionnellement au carre (Quadratiquement avec la taille de l'entree) de la taille de l'entree. Cela signifie que si la taille de l'entree double,
            le temps d'execution quadruple.
            NB: Les boucles imbriquees sont souvent la cause de la complexite quadratique.
            Exemple : 
                > Algorithme de tri par bulle,
                > Algorithme de tri par insertion,
                > Algorithme de tri par selection,
                > Multiplication des matrices carrées,
                
        e- Complexite exponentielle O(2^n) :
            Le temps d'execution croît de manière exponentielle avec la taille de l'entree. Ces algorithmes deviennent rapidement impraticables (tres lents) pour des tailles d'entree relativement petites.
            Exemple : 
                > Resolution du probleme du voyageur de commerce par force brute.
                > Generation de toutes les combinaisons possibles d'un ensemble.
                > Resolution de certains problemes de satisfaction de contraintes.
                > Certains algorithmes de cryptographie.
        