package download

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"net/url"
)

type SitesResponseSite struct {
	Username string
	Name     string
	DBs      []SitesResponseDB
}

type SitesResponseDB struct {
	Type   string
	Folder string
}

func RequestSites() ([]SitesResponseSite, error) {
	url := "https://squig.link/squigsites.json"
	response, err := http.Get(url)
	if err != nil {
		return nil, err
	}
	defer response.Body.Close()
	fmt.Printf("%v - %v\n", url, response.StatusCode)

	body, err := io.ReadAll(response.Body)
	if err != nil {
		return nil, err
	}

	sites := []SitesResponseSite{}
	err = json.Unmarshal(body, &sites)
	if err != nil {
		return nil, err
	}

	return sites, nil
}

type BrandsResponseBrand struct {
	Name   string
	Phones []BrandsResponsePhone
}

type BrandsResponsePhone struct {
	Name          Name
	File          SometimesStringSlice
	Suffix        SometimesStringSlice
	PreferredShop *string
	ReviewScore   ReviewScore
	ReviewLink    *string
	Price         *string
	ShopLink      *string
	Amazon        *string
	AliExpress    *string
}

type Name struct {
	Text string
}

func (name *Name) UnmarshalJSON(data []byte) error {
	if len(data) == 0 {
		return nil
	}
	if data[0] == '"' {
		return json.Unmarshal(data, &name.Text)
	}
	if data[0] == '[' {
		end := len(string(data)) - 2
		*name = Name{Text: string(data[2:end])}
		return nil
	}

	panic(string(data))
}

type ReviewScore struct {
	Text string
}

func (reviewScore *ReviewScore) UnmarshalJSON(data []byte) error {
	if len(data) == 0 {
		return nil
	}
	if data[0] == '"' {
		return json.Unmarshal(data, &reviewScore.Text)
	} else {
		*reviewScore = ReviewScore{Text: string(data)}
		return nil
	}
}

type SometimesStringSlice struct {
	Slice []string
}

func (sometimesStringSlice *SometimesStringSlice) UnmarshalJSON(data []byte) error {
	if len(data) == 0 {
		return nil
	}
	if data[0] == '[' {
		return json.Unmarshal(data, &sometimesStringSlice.Slice)
	} else {
		end := len(string(data)) - 1
		*sometimesStringSlice = SometimesStringSlice{Slice: []string{string(data[1:end])}}
		return nil
	}
}

func requestBrands(username string, folder string) ([]BrandsResponseBrand, error) {
	url := fmt.Sprintf("https://%v.squig.link%vdata/phone_book.json", username, folder)
	response, err := http.Get(url)
	if err != nil {
		return nil, err
	}
	defer response.Body.Close()
	fmt.Printf("%v - %v\n", url, response.StatusCode)

	body, err := io.ReadAll(response.Body)
	if err != nil {
		return nil, err
	}

	brands := []BrandsResponseBrand{}
	err = json.Unmarshal(body, &brands)
	if err != nil {
		return nil, err
	}

	return brands, nil
}

func requestFile(username string, folder string, file string) (*string, error) {
	url := fmt.Sprintf("https://%v.squig.link%v/data/%v", username, folder, url.PathEscape(file))
	response, err := http.Get(url)
	if err != nil {
		return nil, err
	}
	defer response.Body.Close()
	fmt.Printf("%v - %v\n", url, response.StatusCode)

	if !(response.StatusCode >= 200 && response.StatusCode <= 299) {
		return nil, fmt.Errorf("received %v in the response", response.StatusCode)
	}

	body, err := io.ReadAll(response.Body)
	if err != nil {
		return nil, err
	}

	result := string(body)
	return &result, nil
}
