<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>usersDetailUsingGET</name>
   <tag></tag>
   <elementGuidId>da3f02be-148d-43ed-ab2a-48cda1e0bb46</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-type</name>
      <type>Main</type>
      <value>*/*</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://localhost:8181/users/${id}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>2</defaultValue>
      <description></description>
      <id>661097ba-e06d-4681-af51-27e0de66df4b</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <variables>
      <defaultValue>'Reginald Bogisich'</defaultValue>
      <description></description>
      <id>40c52a85-4c40-46aa-9bcd-769b38bdedde</id>
      <masked>false</masked>
      <name>name</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

def variables = request.getVariables()
def id = variables.get('id')

if (id ==1){
	
		WS.verifyElementPropertyValue(response, 'id', 1)
		WS.verifyElementPropertyValue(response, 'name', &quot;Reginald Bogisich&quot;)
		WS.verifyElementPropertyValue(response, 'gender', &quot;F&quot;)
		WS.verifyElementPropertyValue(response, 'image', &quot;/assets/image/cat1.jpg&quot;)
	}
else if(id ==2){
	WS.verifyElementPropertyValue(response, 'id', 1)
}



</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
