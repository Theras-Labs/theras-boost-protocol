package main

import (
	"encoding/json"
	"log"
	"net/http"
)

type Event struct {
	ProjectKey string                 `json:"projectKey"`
	UserId     string                 `json:"userId"`
	Event      string                 `json:"event"`
	Props      map[string]any         `json:"props"`
}

type Distribute struct {
	ProjectKey string `json:"projectKey"`
	UserId     string `json:"userId"`
	Token      string `json:"token"`
	Amount     int64  `json:"amount"`
}

func handleEvents(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	var e Event
	_ = json.NewDecoder(r.Body).Decode(&e)
	w.Header().Set("Content-Type", "application/json")
	_, _ = w.Write([]byte(`{"ok":true}`))
}

func handleDistribute(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	var d Distribute
	_ = json.NewDecoder(r.Body).Decode(&d)
	w.Header().Set("Content-Type", "application/json")
	_, _ = w.Write([]byte(`{"ok":true}`))
}

func handleWallet(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	// In a real service, lookup balances by projectKey + userId.
	// Here we return static demo data.
	resp := map[string]any{
		"tgem":    120,
		"partner": 45,
	}
	w.Header().Set("Content-Type", "application/json")
	_ = json.NewEncoder(w).Encode(resp)
}

func main() {
	http.HandleFunc("/events", handleEvents)
	http.HandleFunc("/distribute", handleDistribute)
	http.HandleFunc("/wallet", handleWallet)
	log.Fatal(http.ListenAndServe(":8080", nil))
}
