//============================================================================//
//                                                                            //
//                         Copyright © 2015 Sandpolis                         //
//                                                                            //
//  This source file is subject to the terms of the Mozilla Public License    //
//  version 2. You may not use this file except in compliance with the MPL    //
//  as published by the Mozilla Foundation.                                   //
//                                                                            //
//============================================================================//

plugins {
	id("sandpolis-module")
	id("sandpolis-soi")
	id("sandpolis-publish")
}

val buildLinuxAmd64 by tasks.creating(Exec::class) {
	workingDir(project.getProjectDir())
	commandLine(listOf("cross", "build", "--release", "--target=x86_64-unknown-linux-gnu"))
}
tasks.findByName("build")?.dependsOn(buildLinuxAmd64)

val buildLinuxAarch64 by tasks.creating(Exec::class) {
	workingDir(project.getProjectDir())
	commandLine(listOf("cross", "build", "--release", "--target=aarch64-unknown-linux-gnu"))
}
tasks.findByName("build")?.dependsOn(buildLinuxAarch64)

val buildWindowsAmd64 by tasks.creating(Exec::class) {
	workingDir(project.getProjectDir())
	commandLine(listOf("cross", "build", "--release", "--target=x86_64-pc-windows-gnu"))
}
tasks.findByName("build")?.dependsOn(buildWindowsAmd64)

publishing {
	publications {
		create<MavenPublication>("mavenLinuxAmd64") {
			groupId = "com.sandpolis"
			artifactId = "distagent-linux-amd64"
			version = project.version.toString()

			artifact(project.file("target/x86_64-unknown-linux-gnu/release/distagent"))
		}
		tasks.findByName("publishMavenLinuxAmd64PublicationToGitHubPackagesRepository")?.dependsOn(buildLinuxAmd64)

		create<MavenPublication>("mavenLinuxAarch64") {
			groupId = "com.sandpolis"
			artifactId = "distagent-linux-aarch64"
			version = project.version.toString()

			artifact(project.file("target/aarch64-unknown-linux-gnu/release/distagent"))
		}
		tasks.findByName("publishMavenLinuxAarch64PublicationToGitHubPackagesRepository")?.dependsOn(buildLinuxAarch64)

		create<MavenPublication>("mavenWindowsAmd64") {
			groupId = "com.sandpolis"
			artifactId = "distagent-windows-amd64"
			version = project.version.toString()

			artifact(project.file("target/x86_64-pc-windows-gnu/release/distagent.exe"))
		}
		tasks.findByName("publishMavenWindowsAmd64PublicationToGitHubPackagesRepository")?.dependsOn(buildWindowsAmd64)
	}
}

tasks.findByName("clean")?.doLast {
	delete("target")
}
