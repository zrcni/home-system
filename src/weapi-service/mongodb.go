package main

import (
	"context"
	"crypto/tls"
	"log"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

type MongoDBWeatherRepo struct {
	db *mongo.Collection
}

func NewMongoDBWeatherRepo(mongoUri string, mongoDb string, mongoCollection string, mongoUser string, mongoPassword string) (*MongoDBWeatherRepo, error) {
	// create a context
	ctx := context.Background()

	// create a mongo client
	var clientOptions *options.ClientOptions
	if mongoUser == "" && mongoPassword == "" {
		clientOptions = options.Client().ApplyURI(mongoUri)
	} else {
		clientOptions = options.Client().ApplyURI(mongoUri).
			SetAuth(options.Credential{
				AuthSource: mongoDb,
				Username:   mongoUser,
				Password:   mongoPassword,
			}).
			SetTLSConfig(&tls.Config{InsecureSkipVerify: false})
	}

	mongoClient, err := mongo.Connect(ctx, clientOptions)
	if err != nil {
		log.Printf("failed to connect to mongodb: %s", err)
		return nil, err
	}

	err = mongoClient.Ping(ctx, nil)
	if err != nil {
		log.Printf("failed to ping database: %s", err)
		return nil, err
	} else {
		log.Printf("pong from database")
	}

	// get a handle for the collection
	collection := mongoClient.Database(mongoDb).Collection(mongoCollection)
	//defer collection.Database().Client().Disconnect(context.Background())

	return &MongoDBWeatherRepo{collection}, nil
}

func (r *MongoDBWeatherRepo) GetObservation(location string) (WeatherObservation, error) {
	var ctx = context.TODO()

	filter := bson.D{{Key: "location", Value: bson.D{{Key: "$eq", Value: location}}}}

	singleResult := r.db.FindOne(ctx, filter)

	var observation WeatherObservation
	err := singleResult.Decode(&observation)
	if err != nil {
		log.Printf("Failed to decode Weather: %s", err)
		return observation, err
	}

	return observation, nil
}

func (r *MongoDBWeatherRepo) InsertObservation(observation WeatherObservation) error {
	ctx := context.TODO()

	_, err := r.db.InsertOne(ctx, observation)
	if err != nil {
		log.Printf("Failed to insert weather observation: %s", err)
		return err
	}

	return nil
}
