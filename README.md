# Litchi Mission Converter

While Parrot's FreeFlight application allows creation of flight plans to automate flights, it seemed easier from an user experience standpoint to be able to plan those automated flights from a device with a bigger screen and a mouse to point and click: a computer.

The Litchi Mission Hub (https://flylitchi.com/hub) seemed appropriate enought to plan missions, however none of the export formats are compatible with Parrot's FreeFlight. Indeed, exports are either CSV or KML but FreeFlight use a specific JSON format. Thus a conversion tool was necessary: here comes `litchi2fp` !

`litchi2fp` aims to convert CSV and KML exports from Litchi to FreeFlight's JSON format.


## KML conversion

KML export in Litchi Mission Hub is primarily aimed at vizualizing the flight plan in 3D using Google Earth. However KML is plain and simple XML at its heart and thus it can be easily parsed. From the KML export `litchi2fp` is able retrieve the following :
 * Mission name
 * Start point and End point
 * Full path of the flight. **Beware: when using the Curved Turns settings in Litchi Mission Hub, all curve points are exported thus creating a flightplan with potentially hundreds of waypoints. While this looks nice in Google Earth, experiments show that FreeFlight just crashes because of the shear amount of waypoints!**

The following informations are lacking from the KML export thus those are not available for `litchi2fp` to translate when using a KML file:
 * Points of Interest
 * Actions
 * Heading and gimbal information
 * Date of creation

## CSV conversion

CSV exports are way more suited for conversion as the information they provide almost map 1:1 with FreeFlight's model.
In particular, mission name and date of creation are missing and must be provided to `litchi2fp` (although current is used for date of creation and `litchi2fp` is able to use the file name as title for the mission).
It is important to note that although action information is present in the CSV export, they do not exactly map 1:1 with FreeFlight's model. More tests are needed to understand the full range of differences, before devising solutions to compensate those.


  # DISCLAIMER

  **`litchi2fp` OUTPUT HAS NOT YET BEEN TESTED WITH A LIVE DRONE. EVEN IF IT HAD BEEN, I AM IN NO WAY RESPONSIBLE FOR ANY DAMAGES DONE TO YOUR DRONE, OR DAMAGES CAUSED BY YOUR DRONE TO THINGS OR PEOPLE, WHEN USING A FLIGHT PLAN PRODUCED BY `litchi2fp` (See Sections 15, 16 of the LICENSE).**