package main

import (
	"bufio"
	"context"
	"encoding/base64"
	"encoding/json"
	"errors"
	"fmt"
	"io"

	"github.com/docker/docker/api/types"
	"github.com/docker/docker/api/types/registry"
	"github.com/docker/docker/client"
)

type ErrorLine struct {
	Error       string      `json:"error"`
	ErrorDetail ErrorDetail `json:"errorDetail"`
}

type ErrorDetail struct {
	Message string `json:"message"`
}

func pushImage(repository, username, password, token string) error {
	ctx := context.Background()

	var authConfig = registry.AuthConfig{
		ServerAddress: repository,
		Username:      username,
		Password:      password,
		IdentityToken: token,
	}

	authConfigBytes, _ := json.Marshal(authConfig)
	authConfigEncoded := base64.URLEncoding.EncodeToString(authConfigBytes)

	dockerClient, err := client.NewClientWithOpts(client.FromEnv, client.WithAPIVersionNegotiation())
	if err != nil {
		return err
	}

	tag := repository + ":latest" //TODO - do not hardcode
	opts := types.ImagePushOptions{
		RegistryAuth: authConfigEncoded,
	}

	reader, err := dockerClient.ImagePush(ctx, tag, opts)
	if err != nil {
		return err
	}
	defer reader.Close()

	err = uploadProgress(reader)
	if err != nil {
		return err
	}

	return nil
}

func uploadProgress(reader io.Reader) error {
	var lastLine string

	scanner := bufio.NewScanner(reader)
	for scanner.Scan() {
		lastLine = scanner.Text()
		fmt.Println(scanner.Text())
	}

	errLine := &ErrorLine{}
	err := json.Unmarshal([]byte(lastLine), errLine)

	if err != nil {
		return err
	}

	if errLine.Error != "" {
		return errors.New(errLine.Error)
	}

	if err = scanner.Err(); err != nil {
		return err
	}

	return nil
}
