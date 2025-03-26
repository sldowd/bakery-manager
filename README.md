# 🧁 Bakery Manager - Rust Portfolio App

A local-first, offline inventory, recipe, and bookkeeping app for small bakeries — built in **Rust** as a full-featured portfolio project.

---

## 📌 Project Vision
This app is designed to:
- Track **ingredients and inventory**
- Save and scale **recipes**
- Log **sales and expenses** for basic accounting
- Work entirely **offline**, storing data locally using SQLite
- Be packaged into a desktop app (GUI coming after MVP)

---

## 🚀 Tech Stack (Planned)

| Component        | Technology              | Notes                                |
|------------------|--------------------------|--------------------------------------|
| Language         | Rust 🦀                  | Fast, safe, and modern                |
| Database         | SQLite                   | Local storage, no server required     |
| GUI (post-MVP)   | Tauri or Iced            | Desktop-native or hybrid frontend     |
| Packaging        | Cargo + Tauri (later)    | To create a .app file for macOS       |
| Version Control  | Git + GitHub             | Public portfolio and changelog        |

---

## ✅ MVP Features

- [ ] Create and update **inventory items**
- [ ] Store and retrieve **recipes**
- [ ] Calculate cost per recipe or unit
- [ ] Track **expenses and income**
- [ ] Export reports (CSV or JSON)
- [ ] Clean CLI interface

---

## 🗃 Planned Database Schema

### `inventory`
| Field              | Type      |
|--------------------|-----------|
| id (PK)            | INTEGER   |
| name               | TEXT      |
| unit               | TEXT      |
| quantity           | REAL      |
| cost_per_unit      | REAL      |

### `recipes`
| Field              | Type      |
|--------------------|-----------|
| id (PK)            | INTEGER   |
| name               | TEXT      |
| instructions       | TEXT      |
| yield              | INTEGER   |

### `recipe_ingredients`
| Field              | Type      |
|--------------------|-----------|
| recipe_id (FK)     | INTEGER   |
| ingredient_id (FK) | INTEGER   |
| quantity_required  | REAL      |

### `transactions`
| Field              | Type      |
|--------------------|-----------|
| id (PK)            | INTEGER   |
| date               | TEXT      |
| type               | TEXT      |
| amount             | REAL      |
| description        | TEXT      |

---

## 📂 Folder Structure (planned)
```
src/
├── main.rs
├── db.rs        # SQLite interface
├── models.rs    # Structs for inventory, recipes, etc.
├── cli.rs       # Command-line interface logic
```

---

## ✨ Future Enhancements

- [ ] GUI interface using Tauri (Rust + Web frontend)
- [ ] Barcode scanning (e.g. via mobile app or QR code)
- [ ] Cloud backup/sync (optional)
- [ ] Ingredient usage projections
- [ ] User roles or multi-user support

---

## 👩‍🍳 Creator
**Sarah Dowd**  
CS @ SNHU | Aspiring Software Engineer | Baker & Entrepreneur  
[GitHub Profile](https://github.com/sldowd)

---

## 📝 License
MIT License — Free to use and modify

---

> Built for passion, practice, and pastries 🍞

