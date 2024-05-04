package download

import (
	"fmt"

	"gorm.io/gorm"

	"wobblingstatistics/internal/database"
)

type Squig struct {
	Folder   string
	SiteID   uint
	Username string
}

func Main() {
	db, err := database.Create("squig.db")
	if err != nil {
		panic(err)
	}

	sites, err := RequestSites()
	if err != nil {
		panic(err)
	}
	db.Transaction(func(tx *gorm.DB) error {
		for _, site := range sites {
			var siteRecord database.Site
			tx.FirstOrCreate(&siteRecord, database.Site{
				Name:     site.Name,
				Username: site.Username,
			})
			for _, db := range site.DBs {
				var dbRecord database.DB
				tx.FirstOrCreate(&dbRecord, database.DB{
					Folder: db.Folder,
					SiteID: siteRecord.ID,
					Type:   db.Type,
				})
			}
		}
		return nil
	})

	db.Transaction(func(tx *gorm.DB) error {
		squigs := []Squig{}
		tx.
			Model(&database.Site{}).
			Joins("JOIN dbs ON dbs.site_id = sites.id").
			Select("dbs.folder, dbs.site_id, sites.username").
			Scan(&squigs)
		for _, squig := range squigs {
			brands, err := requestBrands(squig.Username, squig.Folder)
			if err != nil {
				panic(err)
			}
			for _, brand := range brands {
				var brandRecord database.Brand
				tx.FirstOrCreate(&brandRecord, database.Brand{
					Name:   brand.Name,
					SiteID: squig.SiteID,
				})
				for _, phone := range brand.Phones {
					var phoneRecord database.Phone
					tx.FirstOrCreate(&phoneRecord, database.Phone{
						Amazon:        phone.Amazon,
						BrandID:       brandRecord.ID,
						Name:          phone.Name.Text,
						PreferredShop: phone.PreferredShop,
						Price:         phone.Price,
						ReviewLink:    phone.ReviewLink,
						ReviewScore:   phone.ReviewScore.Text,
						ShopLink:      phone.ShopLink,
					})
					for _, file := range phone.File.Slice {
						channelLeft, _ := requestFile(squig.Username, squig.Folder, fmt.Sprintf("%v L.txt", file))
						channelRight, _ := requestFile(squig.Username, squig.Folder, fmt.Sprintf("%v R.txt", file))
						channelUnknown, _ := requestFile(squig.Username, squig.Folder, fmt.Sprintf("%v.txt", file))
						var fileRecord database.File
						tx.FirstOrCreate(&fileRecord, database.File{
							ChannelLeft:    channelLeft,
							ChannelRight:   channelRight,
							ChannelUnknown: channelUnknown,
							PhoneID:        phoneRecord.ID,
							Text:           file,
						})
					}
					for _, suffix := range phone.Suffix.Slice {
						var suffixRecord database.Suffix
						tx.FirstOrCreate(&suffixRecord, database.Suffix{
							PhoneID: phoneRecord.ID,
							Text:    suffix,
						})
					}
				}
			}
		}
		return nil
	})
}
