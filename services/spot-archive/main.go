package main

import (
	"context"
	"fmt"
	"os"
	"strconv"
	"strings"
	"time"

	"github.com/aws/aws-sdk-go/aws"
	"github.com/aws/aws-sdk-go/aws/session"
	"github.com/aws/aws-sdk-go/service/ec2"
	"github.com/jackc/pgx/v4"
)

var AWSRegions = []string{
	"us-east-1",
	"us-east-2",
	"us-west-1",
	"us-west-2",
	"af-south-1",
	"ap-south-1",
	"ap-south-2",
	"ap-east-1",
	"ap-southeast-1",
	"ap-southeast-2",
	"ap-southeast-3",
	"ap-southeast-4",
	"ap-northeast-1",
	"ap-northeast-2",
	"ap-northeast-3",
	"ca-central-1",
	"eu-central-1",
	"eu-central-2",
	"eu-west-1",
	"eu-west-2",
	"eu-west-3",
	"eu-south-1",
	"eu-south-2",
	"eu-north-1",
	"il-central-1",
	"me-south-1",
	"me-central-1",
	"sa-east-1",
}

type SpotInstance struct {
	InstanceType string
	SpotPrice    float64
}

type SpotInstancePrice struct {
	Price            float64
	AvailabilityZone string
}

func fetchSpotInstancePrices(region string, instanceTypes []string) (map[string][]SpotInstancePrice, error) {
	sess, err := session.NewSession(&aws.Config{
		Region: aws.String(region),
	})
	if err != nil {
		return nil, err
	}

	svc := ec2.New(sess)

	now := time.Now().UTC()

	input := &ec2.DescribeSpotPriceHistoryInput{
		InstanceTypes: aws.StringSlice(instanceTypes),
		ProductDescriptions: []*string{
			aws.String("Linux/UNIX"),
		},
		MaxResults: aws.Int64(1000),
		StartTime:  &now,
	}

	result, err := svc.DescribeSpotPriceHistory(input)
	if err != nil {
		return nil, err
	}

	prices := make(map[string][]SpotInstancePrice)
	for _, spotPrice := range result.SpotPriceHistory {
		priceStr := aws.StringValue(spotPrice.SpotPrice)
		price, err := strconv.ParseFloat(priceStr, 64)
		if err != nil {
			continue
		}
		instanceType := aws.StringValue(spotPrice.InstanceType)
		availabilityZone := aws.StringValue(spotPrice.AvailabilityZone)
		prices[instanceType] = append(prices[instanceType], SpotInstancePrice{Price: price, AvailabilityZone: availabilityZone})
	}

	return prices, nil
}

func spotPricingForForecast(conn *pgx.Conn, region string, instanceTypes []string) error {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*30)
	defer cancel()

	tx, err := conn.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	prices, err := fetchSpotInstancePrices(region, instanceTypes)
	if err != nil {
		return err
	}

	// Prepare a slice to hold values to be inserted
	valuesToInsert := []string{}

	// Iterate over each instance type to prepare the values to be inserted
	for _, instanceType := range instanceTypes {
		if spotInstancePrices, ok := prices[instanceType]; ok {
			for _, spotInstancePrice := range spotInstancePrices {
				// Check if this specific entry already exists with the same price
				var existingPrice float64
				err := tx.QueryRow(ctx, `
				SELECT price_per_hour FROM spot_archive
				WHERE region = $1 AND availability_zone = $2 AND instance_type = $3
				ORDER BY timestamp DESC LIMIT 1`,
					region, spotInstancePrice.AvailabilityZone, instanceType).Scan(&existingPrice)

				// If the entry does not exist or the price is different, prepare it for insertion
				if err == pgx.ErrNoRows || (err == nil && existingPrice != spotInstancePrice.Price) {
					valueStr := fmt.Sprintf("('%s', '%s', '%s', %f, NOW())", region, spotInstancePrice.AvailabilityZone, instanceType, spotInstancePrice.Price)
					valuesToInsert = append(valuesToInsert, valueStr)
				} else if err != nil {
					return err
				}
			}
		}
	}

	// If there are new or changed prices, insert them
	if len(valuesToInsert) > 0 {
		fmt.Printf("Writing %d changed prices for %s.\n", len(valuesToInsert), region)
		insertQuery := fmt.Sprintf("INSERT INTO spot_archive (region, availability_zone, instance_type, price_per_hour, timestamp) VALUES %s",
			strings.Join(valuesToInsert, ", "))
		_, err = tx.Exec(ctx, insertQuery)
		if err != nil {
			return err
		}
	}

	if err = tx.Commit(ctx); err != nil {
		return err
	}

	return nil
}

func main() {
	// Connect to the database
	dbURL := os.Getenv("DB_URL")

	if dbURL == "" {
		fmt.Println("DB_URL environment variable is not set.")
		return
	}

	conn, err := pgx.Connect(context.Background(), dbURL)
	if err != nil {
		fmt.Println("Failed to connect:", err)
		return
	}

	// Instance types you're interested in
	instanceTypes := []string{"t4g.small", "t3.micro", "t2.micro", "t4g.micro", "t3a.micro"}

	// Loop through each region and save the latest prices
	for {
		for _, region := range AWSRegions {
			if err := spotPricingForForecast(conn, region, instanceTypes); err != nil {
				fmt.Printf("Failed to update spot pricing for %s: %s\n", region, err)
			} else {
				fmt.Printf("Spot pricing updated successfully for %s.\n", region)
			}
		}

		time.Sleep(2 * time.Minute)
	}
}
