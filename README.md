# PROG64 – Linear Genetic Programming sur CartPole



Projet Rust réalisé dans le cadre du module PROG64 à l'INSA Rennes.



L'objectif est de développer un système de **Linear Genetic Programming (LGP)** capable de faire évoluer automatiquement des programmes afin de contrôler l'environnement CartPole.



## 1. Présentation



Le projet est organisé en deux crates Rust :



- `env` : implémentation de l'environnement CartPole et du générateur pseudo-aléatoire PCG.

- `prog64` : représentation des programmes, encodage binaire, interpréteur, évaluation et algorithme évolutionnaire.



L'algorithme cherche à produire un programme capable de maintenir un pendule inversé en équilibre sur un chariot.



## 2. Architecture du projet



```text

prog64/

├── Cargo.toml

├── Cargo.lock

├── .gitignore

│

├── env/

│   ├── Cargo.toml

│   └── src/

│       ├── lib.rs

│       ├── cartpole.rs

│       └── pcgrng.rs

│

└── lgp/

    ├── Cargo.toml

    ├── src/

    │   ├── lib.rs

    │   ├── main.rs

    │   ├── instruction.rs

    │   ├── program.rs

    │   ├── interpreter.rs

    │   ├── evaluation.rs

    │   └── evolution.rs

    │

    └── tests/

        └── integration_cartpole.rs

```



## 3. Représentation des programmes



Chaque programme est constitué de cinq instructions exécutées séquentiellement.



Une instruction est encodée sur 11 bits :



- 3 bits pour l'opération ;

- 2 bits pour le registre de destination ;

- 3 bits pour le premier opérande ;

- 3 bits pour le second opérande.



Le programme complet occupe donc 55 bits et peut être stocké dans un entier `u64`.



### Opérations disponibles



| Opération | Description |

|---|---|

| Add | Addition |

| Sub | Soustraction |

| Mul | Multiplication |

| Div | Division protégée |

| Ln | Logarithme naturel protégé |

| Exp | Exponentielle avec limitation |

| Max | Maximum |

| Min | Minimum |



Les instructions utilisent quatre entrées et quatre registres internes.



Les quatre entrées correspondent aux variables d'état du CartPole :



1. Position du chariot.

2. Vitesse du chariot.

3. Angle du pendule.

4. Vitesse angulaire du pendule.



Les registres sont initialisés à zéro. Le résultat final du programme correspond à la valeur du registre `R0`.



## 4. Interprétation et contrôle du CartPole



À chaque étape de simulation, le programme reçoit les quatre variables d'état de l'environnement.



L'interpréteur exécute les cinq instructions et produit une valeur numérique.



Cette valeur détermine l'action appliquée au chariot :



- Résultat positif : déplacement vers la droite.

- Résultat nul ou négatif : déplacement vers la gauche.



Une récompense de 1 est attribuée pour chaque étape pendant laquelle le système reste dans les limites autorisées.



Un épisode peut durer jusqu'à 500 étapes.



## 5. Algorithme évolutionnaire



L'évolution repose sur une population de programmes représentés par des individus.



### Initialisation



Les programmes de la population initiale sont générés aléatoirement. Seuls les 55 bits utiles sont conservés.



### Évaluation



Chaque individu est évalué sur plusieurs épisodes de CartPole. Son score correspond à la moyenne des récompenses obtenues.



### Sélection



Les individus sont triés par score décroissant. Les meilleurs sont conservés en fonction du taux de survie défini.



### Élitisme



Les individus sélectionnés sont directement copiés dans la génération suivante afin de préserver les meilleurs programmes.



### Reproduction et mutation



Pour compléter la population, un parent est choisi aléatoirement parmi les survivants.



Son programme est copié, puis chaque bit des 55 bits peut être inversé indépendamment selon la probabilité de mutation.



La reproduction utilise donc une mutation bit à bit et ne fait pas intervenir de croisement entre deux parents.



### Répétition



Les étapes d'évaluation, de sélection et de reproduction sont répétées jusqu'à atteindre le nombre de générations demandé.



Le meilleur programme rencontré pendant toute l'évolution est conservé.



## 6. Reproductibilité



Le projet utilise un générateur pseudo-aléatoire PCG (`PCGRng`).



La graine (`seed`) permet de contrôler les séquences pseudo-aléatoires utilisées pour générer les programmes et effectuer l'évolution.



À configuration et graine identiques, l'exécution est déterministe.



Cette propriété permet notamment de comparer des expériences et de vérifier la reproductibilité des résultats.



## 7. Paramètres de l'exécution principale



Les paramètres actuellement définis dans `main.rs` sont :



| Paramètre | Valeur |

|---|---:|

| Taille de la population | 120 |

| Nombre de générations | 40 |

| Taux de survie | 0,20 |

| Taux de mutation | 0,02 |

| Épisodes par individu | 100 |

| Graine d'évolution | 10099 |

| Épisodes de validation indépendante | 100 |

| Graine de validation | 1000 |



L'évolution réalise ainsi 4 800 évaluations de programmes, chacune utilisant 100 épisodes.



Après l'évolution, le meilleur programme est évalué sur 100 épisodes supplémentaires avec une graine distincte.



Le programme affiche notamment :

- Le meilleur score obtenu.

- La génération à laquelle le meilleur programme a été découvert.

- Le nombre total d'évaluations.

- L'encodage binaire du meilleur programme.

- Les cinq instructions du programme.

- Les statistiques de validation : moyenne, minimum, maximum et écart-type.

- Le nombre et le taux d'épisodes parfaits.



## 8. Compilation et exécution



Depuis la racine du dépôt :



### Compiler le projet



```powershell

cargo build --workspace

```



### Exécuter les tests



```powershell

cargo test --workspace

```



### Vérifier le formatage



```powershell

cargo fmt --all -- --check

```



### Vérifier le code avec Clippy



```powershell

cargo clippy --workspace --all-targets -- -D warnings

```



### Lancer l'évolution en mode optimisé



```powershell

cargo run -p prog64 --release

```



Attention : l'exécution principale effectue plusieurs milliers d'évaluations de programmes sur CartPole et peut donc prendre du temps.



## 9. Tests



Le projet comporte des tests unitaires et des tests d'intégration.



Ils vérifient notamment :



- L'encodage et le décodage des instructions.

- La validité de l'encodage des programmes.

- L'exécution des programmes.

- Le comportement de l'évaluation.

- La reproductibilité de l'évolution.

- Le rejet des paramètres invalides.

- L'utilisation de l'API publique pour construire, encoder et exécuter un programme.

- La reproductibilité des statistiques d'évaluation sur CartPole.



Pour lancer l'ensemble des tests :



```powershell

cargo test --workspace

```



## 10. Technologies



- **Rust** : langage de programmation.

- **Cargo** : gestion du workspace, compilation et tests.

- **CartPole** : environnement de simulation et d'évaluation.

- **PCG** : générateur pseudo-aléatoire.

- **Linear Genetic Programming** : méthode d'évolution des programmes.



---



Projet développé dans le cadre du module PROG64 – INSA Rennes.
