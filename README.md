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

W przyszłej wersji chciałbym dodać dockera.
