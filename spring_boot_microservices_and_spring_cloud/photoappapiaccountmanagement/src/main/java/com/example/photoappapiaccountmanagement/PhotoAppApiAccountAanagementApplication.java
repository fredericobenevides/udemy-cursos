package com.example.photoappapiaccountmanagement;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.cloud.client.discovery.EnableDiscoveryClient;
import org.springframework.cloud.netflix.eureka.EnableEurekaClient;

@SpringBootApplication
@EnableDiscoveryClient
public class PhotoAppApiAccountAanagementApplication {

	public static void main(String[] args) {
		SpringApplication.run(PhotoAppApiAccountAanagementApplication.class, args);
	}

}
