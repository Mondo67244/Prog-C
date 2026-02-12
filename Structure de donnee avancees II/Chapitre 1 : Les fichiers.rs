 Introduction: 
Un fichier est un ensemble structure d'informations stockees sur un support de stockage (Disques Dur, Cles Usb, Cloud... ). 
Il permet  de conserver les donnees de maniere organisees afin de pouvoir les Consulter, les Modifier ou les Transmettres Ulterieurement.
    
L'un des roles essentiesl d'un fichier c'est d'assurer la persistance des donnees (reste stockee meme apres la fermeture d'un programme). 
- Une donnee persistante : est celle qui continue d'exister meme apres la fermeture d'un programme ou l'arret d'un ordinateur (ex. Cle Usb) 
contrairement aux donnees stockees temporairement en memoire vive qui disparaissent lorsque le systeme s'eteint.

   "Les fichiers constituent donc un mecanisme fondamental de stockage permanent", comme exemple on a : 
    - Les documents Textes enregistres
    - Une image sauvegardee
    - Une base de donnee

    I - Definition et declaration d'un fichier : 

Un fichier est un ensemble structure de donnees sauvegardees afin d'etre conserve de maniere persistante. 
Il permet : 

 -  Stocker les informations de facon durable
 -  D'organiser les donnees
 -  De les Lires , les Modifier ou les reutiliser ulterieurement.

Un fichier possede donc generalement les elements suivants : 

 * Un Nom : pour identifier le fichier parmi d'autres
 * Une extension : pour specifier le type d'editeur
 * Un emplacement (Le Chemin d'Access) : pour localiser l'emplacement de stockage
 * Un type de donnees  (texte , binaire, image, video...) 

     1 - Declaration : 
En programmation la declaration d'un fichier consiste a creer une nouvelle variable qui va permettre de manipuler ce fichier (lecture, ecriture ,modification)
On declare une variable de type fichier via la syntaxe suivante :

    "TypeFichier nomFichier "

Avant d'utiliser un fichier on doit au prealable effectuer les opertaions suivantes : 

 - La declarer
 - L'ouvrir
 - L'utiliser (lire / ecrire)
 - La Fermer (pour eviter les fuites de memoire)

 Remarque : l'ouverture d'un fichier necessite trois modes specifiques a savoir :  

    "La lecture , L'ecriture et l'ajout (modification)"

*EXEMPLE D'OUVERTURE D'UN FICHIER EN LANGAGE C : 
--legende : r = read , w = write , a = add ; 

    FILE * fichier; //Declaration
    fichier = fopen("nomfichier.extension", r);//Ouvrir le fichier en precisant le mode , ici ouverture

    
    II - Les operations de base sur les fichiers : 
    ----------------------------------------------

Plusieurs operations sont essentielles lors de la manipulation des fichiers , et les plus fondamentales sont : 

        1 - La declaration d'un fichier : 
    Elle permet de definir une variable de type fichier , la syntaxe est la suivante : 

    //En algorithmique
    Declarer F:fichier

        2 - L'ourverture d'un fichier : 
     !!! On doit toujours ouvrir un fichier avant toute operations. La primitive utilisee est "Ouvrir" ("fopen" en c). 
     On va distinguer trois modes d'ouverture :

     //En algorithmique
     ---> Ouverture en Lecture : 
     Ouvrir(F, "Lecture");
     ---> Ouverture en Ecriture : 
     Ouvrir(F, "Ecriture");
     ---> Ouverture en Ajout : 
     Ouvrir(F, "Ajout");

        3 - Lecture dans un fichier : 
    Elle permet de recuperer les donnees stockees dans un fichier pour des fins utiles. La primitive est "Lire" ("fread" en c). 
    On va distinguer deux variantes pour la lecture :

    ---> Lecture d'un element : 
    Lire(F, variable);
    ---> Lecture jusqu'a la fin du fichier : 
    Tantque nonFin(F) faire
        Lire(F, variable);
        Lire(variable);
    Fintantque
        
         4 - Ecriture dans un fichier : 
    Elle permet d'enregistrer les donnees dans le fichier, la primitive est "Ecrire" ("fwrite" en c),  la syntaxe est : 

    //En algorithmique
    Ecrire(F, donnee); ex. Ecrire(F, "Bonjour");

        5 - Ajout dans un fichier : 
    Cette operation est similaire a celle de l'ecriture mais elle s'effectue sans effacer le contenu existant. 
    La syntaxe est la suivante : 

    Ecrire(F, "Ajout")

        6 - Fermeture d'un fichier : 
    Cette operation est tres cruciale car elle peermet de liberer les ressoruces et d'assurer une sauvegarde correcte des donnnees.(Fclose)
    La syntaxe est : 
    Fermer(F);

    Exemple : Ecrire un algorithme permettant d'ecrire des nombres dans un fichier puis de les lire :

    Algorithme manipulationFichier
        Declarer F:fichier
        Declarer X:entier   
    Debut
    //ouverture du fichier en ecriture
        Ouvrir(F, "Ecriture");
        Pour X <-- 1 a 5 faire
            Ecrire(F, X);
        FinPour
        Fermer(F, X);
    //Ouverture du fichier en lecture
        Ouvrir(F, "Lecture")
        Tantque (nonFin(F)) faire
            Lire(F, X);
            Lire(X);
        Fintantque
            Fermer(F);
    Fin

    Exercice 1 : 
     Ecrire un algorithme qui permet de saisir 05 entiers puis de les enregistrer dans un fichier 
    Exercice 2 : 
     Ecrire un algorithme qui lit tous les nombres du fichier precedent et les affiches a l'ecran
    Exercice 3 :
     Ecrire un algorithme qui calcule la somme des nombres stockes dans un fichier
    Exercice 4 : 
     Ecrire un algorithme qui copie le contenu d'un fichier source vers un fichier destination
    Exercice 5  :
     Ecrire un algorithme qui compte le nombre d'elements contenus dans un fichier
     


     Solution  :


    Algorithme lecture

    Exercices : 

    Algorithme global
        Declarer F,C: fichier
        Declarer X,i,r,val,compt: entier
    Debut
        //EXO 1 : saisie de 05 entiers 
     Ouvrir(F, "Ecriture");
        Pour i allant de 1 a 5  Faire
         Ecrire("Entrer le nombre ",i, " :")
            Lire(X)//lire la variable
            Ecrire(F, X);//ecrire dans le fichier
        FinPour
        Fermer(F);
        //EXO 2 : lire tous les nombres du fichier precedent
         Ouvrir(F, "Lecture");
         Tantque (nonFin(F)) Faire
            Lire(F, X);//lire le fichier
            Ecrire(X);//afficher
         Fintantque
        Fermer(F);
        //EXO 3 : calcul de la somme des nombres dans le fichiers
        r <-- 0;
        Ouvrir(F, "Lecture");
        Tantque (nonFin(F)) Faire
        Lire(F, X);
        r <-- r + X;
        FinPour
        Fermer(F);
        //EXO 4 : Copie du contenu d'un fichier dans un autre
        Ouvrir(F, "Lecture");
        Ouvrir(R, "Ecriture");
        Tantque (nonFin(F)) Faire
            Lire(F, X);//lire le fichier
            val <-- Ecrire(X);
            Ecrire(C, val);//ecrire le contenu de la variable dans le fichier
        Fintantque
        Fermer(F);
        Fermer(R);
        //EXO 5 : Comptage dans le fichier
        compt <-- 0 ;
        Ouvrir(F, "Lecture");
        Tantque (nonFin(F)) Faire
          Lire(F, X);
          compt <-- compt + 1;
        Fintantque
        Fermer(F);
    Fin