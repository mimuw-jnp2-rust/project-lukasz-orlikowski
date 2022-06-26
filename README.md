# Trello

## Autorzy
- Łukasz Orlikowski (gr 9, @lorlikowski)

## Opis
Celem zadania byłoby stworzenie aplikacji WWW służącej optymalizacji, zarządzaniu i planowaniu pracy własnej i zespołu à la Trello z różnymi rozwinięciami. Backend zostałby napisany w Ruście (najpewniej za pomocą frameworku Rocket). Dotego zostałby dołączony front (najpewniej framework yew) oraz testy.
## Funkcjonalność
- Tworzenie tablic (prywatnych i zespołowych) oraz tworzenie i zarządzanie zespołem
- Tworzenie list na tablicy (TODO, Running, itp.)
- Dodawanie/usuwanie/modyfikacja zadań oraz ustawianie deadlinu, przenoszenie pomiędzy listami
- Dodatkowe dane zadań: notatki, miejsce, lista osób przypisanych, lista zadań, możliwość przypisania osoby do konkretnego zadania z listy
- Log zdarzeń na liście (zmiana deadlinu, tytułu, dodanie osoby)
- Każdy użytkownik może startować swoje różne timery (np. do pomiaru czasu pracy)
- Historia timerów
- Tagowanie zadań i punkty funkcyjne
- Filtracja zadań po osobach. tagach, deadlinach, punktach funkcyjnych
- Dodawanie milestonów

## Propozycja podziału na części
- pierwsza część:
  -Tworzenie tablic (prywatnych i zespołowych) oraz tworzenie i zarządzanie zespołem
  - Tworzenie list na tablicy (TODO, Running, itp.)
  - Dodawanie/usuwanie/modyfikacja zadań oraz ustawianie deadlinu, przenoszenie pomiędzy listami
  - Dodatkowe dane zadań: notatki, miejsce, lista osób przypisanych, lista zadań, możliwość przypisania osoby do konkretnego zadania z listy

- druga część:
  - Log zdarzeń na liście (zmiana deadlinu, tytułu, dodanie osoby)
  - Każdy użytkownik może startować swoje różne timery (np. do pomiaru czasu pracy)
  - Historia timerów
  - Tagowanie zadań i punkty funkcyjne
  - Filtracja zadań po osobach. tagach, deadlinach, punktach funkcyjnych
  - Dodawanie milestonów

## Biblioteki
- Rocket (api)
- yew (frontend)
- Diesel (ORM do łączenia się z bazą danych (postgres) https://diesel.rs/)

## Co udało się zrobić w pierwszej części?

Udało się zrealizować tworzenie/modyfikacja/usuwanie tablic prywatnych i zespołowych, tworzenie zespołu, tworzenie/modyfikacja/usuwanie list na tablicy, tworzenie/usuwanie/modyfikacja zadań, ustawianie deadlinu, przenoszenie pomiędzy listami oraz dodatkowe dane zadań jak w opisie.

Niestety nie w pełni działa autoryzacja użytkownika na endpointach (JWT jest wymagany ale nie jest sprawdzany). Nie udało mi się tego zrealizować, ponieważ długo zajęła mi nauka pisania w yew (frontend - dziwnie się pisze nie w javascripcie, przynajmniej dla mnie niektóre rzeczy były nieintuicyjne + dość słaba dokumentacja jest dostępna).

Aplikacja jest podzielona zgrubsza na dwie aplikacje frontend i backend. W backendzie mamy migracje napisane w sql oraz katalog src wystawiający api w pliku main. Pozostałe pliki to w większości pliki odpowiedzialne za komunikację z bazą danych. W frontendzie mamy plik to komunikacji z api (api.rs) utils, types oraz folder components z komponentami.

cargo clippy odpalony na częsci backendowej daje trochę warningów. Nie udało mi się ich wyciszyć/rozwiązać a jeżeli dobrze rozumiem pochodzą z #Insertable, #AsChangeSet, więc z kodu niezależnego ode mnie.

Aby odpalić backend
```
rustup default nightly
cargo run
```

Aby odpalić frontend
```
rustup target add wasm32-unknown-unknown

cargo install --locked trunk

trunk serve
```

Na branchu docker jest możliwość odpalenia dockera komendami:
```
docker-compose build
docker-compose up
```
Stworzy on oba serwery (frontend i backend). Powinien działać dobrze, ale testowałem głównie bez dockera, więc w razie problemów z dockerem należy skorzystać z wersji bez docker na branchu main.

## Co udało się zrealizować w drugiej części?

Udało się zrealizować zamierzone funkcjonalności na drugą część zadania oraz dodać trochę unit testów. Niestety nie udało się wprowadzić wszystkich zmian z review z powodu braku czasu.

Jeżeli chodzi o odpalanie aplikacji nie zmieniło się nic względem pierwszej części zadania.

Jeżeli chodzi o unit testy, to nie znalazłem sposobu jak można przygotowywać pod nie środowisko zewnętrzne (to znaczy jak z poziomu Rusta odpalać przed testami jakiś kod konfiguracyjny). W związku z tym migracje bazy danych nie są wkomponowane w testy. Powoduje to w szczególności, że trzeba samemu przygotować bazę danych przed uruchomieniem testów. Przykładowy sposób:
- Skopiować plik z bazą danych (właściwy z serwera) w inne miejsce.
- Odpalić backend za pomocą cargo run (odpali migracje)
- Zakończyć proces backendu i uruchomić cargo test

