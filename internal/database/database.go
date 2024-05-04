package database

import (
	"log"
	"time"

	"gorm.io/driver/sqlite"
	"gorm.io/gorm"
	"gorm.io/gorm/logger"
)

type Site struct {
	gorm.Model
	ID       uint
	Name     string
	Username string
}

type DB struct {
	gorm.Model
	ID     uint
	Folder string
	Site   Site
	SiteID uint
	Type   string
}

type Brand struct {
	gorm.Model
	ID     uint
	Name   string
	Site   Site
	SiteID uint
}

type Phone struct {
	gorm.Model
	ID            uint
	Amazon        *string
	Brand         Brand
	BrandID       uint
	Name          string
	PreferredShop *string
	Price         *string
	ReviewLink    *string
	ReviewScore   string
	ShopLink      *string
}

type File struct {
	gorm.Model
	ID             uint
	ChannelLeft    *string
	ChannelRight   *string
	ChannelUnknown *string
	Phone          Phone
	PhoneID        uint
	Text           string
}

type Suffix struct {
	gorm.Model
	ID      uint
	Phone   Phone
	PhoneID uint
	Text    string
}

func Create(name string) (*gorm.DB, error) {
	db, err := gorm.Open(sqlite.Open(name), &gorm.Config{
		Logger: logger.New(
			log.Default(),
			logger.Config{
				Colorful:                  true,
				IgnoreRecordNotFoundError: true,
				LogLevel:                  logger.Warn,
				SlowThreshold:             200 * time.Millisecond,
			},
		),
	})
	if err != nil {
		return nil, err
	}

	db.AutoMigrate(&Site{})
	db.AutoMigrate(&DB{})
	db.AutoMigrate(&Brand{})
	db.AutoMigrate(&Phone{})
	db.AutoMigrate(&File{})
	db.AutoMigrate(&Suffix{})

	return db, nil
}
